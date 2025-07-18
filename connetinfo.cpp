#include "connetinfo.h"
#include <serial.h>
#include <QThreadPool>
#include <iomanip>


using namespace serial;

QList<QString> ConnetInfo::ports() const
{
    return m_ports;
}

void ConnetInfo::setPorts(const QList<QString> &newPorts)
{

    if (m_ports == newPorts)
        return;
    m_ports = newPorts;
    emit portsChanged();
}
void ConnetInfo::getPorts() {
    if (!m_monitorThread || m_monitorThread->isFinished()) {
        m_monitorThread = new QThread();
        QObject::connect(m_monitorThread, &QThread::started, [this]{
            while (!m_monitorThread->isInterruptionRequested()) {
                std::vector<PortInfo> ports = serial::list_ports();
                QList<QString> list;
                for (const PortInfo &p : ports) {
                    // qDebug() << p.port << p.description << p.hardware_id << "\n";
                    list.append(QString::fromStdString(p.port));
                }
                setPorts(list);
                QThread::msleep(1000);
            }
        });
        m_monitorThread->start();
    }
}
std::string bufferToHex(const uint8_t* buffer, size_t length) {
    std::ostringstream oss;
    oss << std::uppercase << std::hex << std::setfill('0');
    for(size_t i = 0; i < length; ++i) {
        oss << std::setw(2) << static_cast<int>(buffer[i]) << " ";
    }
    return oss.str();
}



class Worker : public QRunnable {
    using Callback = std::function<void(QString)>;

    std::atomic<bool> m_abortFlag{false};  // 原子操作标志位

    public:
    Worker(serial::Serial *connetct_serial, ConnetInfo *connetInfo) {
        t_connetct_serial=connetct_serial;
        m_connetInfo = connetInfo;
    }
    ~Worker(){
        qDebug() << "线程析构析构.....\n";
    };
    void abort() { m_abortFlag = true; }  // 外部调用接口

    void run() override {
        qDebug() << "线程进入.....\n";
        m_abortFlag = false;
        while (!m_abortFlag) {
            try {
                size_t len = t_connetct_serial->read(buffer, sizeof(buffer));
                if (len == 0) continue;
                // qDebug() << t_connetct_serial->available() <<  len << bufferToHex(buffer, len) << "\n";
                auto str = bufferToHex(buffer, len);
                m_connetInfo->dataChanged(QString::fromStdString(str));
            } catch (std::exception) {
                return;
            }
        }
    }

private:
    serial::Serial *t_connetct_serial;
    uint8_t buffer[128];
    ConnetInfo *m_connetInfo;

};

void ConnetInfo::connectPort(QString path, quint32 buad, quint8 data_bit, quint8 stop_bit) {
    qDebug() << path << buad << data_bit << stop_bit << "\n";
    if (connetct_serial) {
        if (connetct_serial->getBaudrate() == buad && connetct_serial->getPort() == path) {
            connetct_serial->open();
        } else {
            delete connetct_serial;
            connetct_serial=nullptr;
            connetct_serial = new serial::Serial(path.toStdString(),buad, serial::Timeout::simpleTimeout(500));
            connetct_serial->setBytesize(eightbits);
            connetct_serial->setStopbits(stopbits_one);
        }
    } else {
        connetct_serial = new serial::Serial(path.toStdString(),buad, serial::Timeout::simpleTimeout(500));
        connetct_serial->setBytesize(eightbits);
        connetct_serial->setStopbits(stopbits_one);
    }

    if (!connectThread) {
        connectThread = new Worker(connetct_serial, this);
        connectThread->setAutoDelete(false);
        QThreadPool::globalInstance()->start(connectThread);
    }
}

void ConnetInfo::disconnectPort(){
    if(connectThread) {
        connetct_serial->close();  // 先关闭串口资源
        // connetct_serial->open()
        Worker* work = static_cast<Worker*>(connectThread);
        work->setAutoDelete(true);
        work->abort();
        delete connectThread;
        connectThread = nullptr;
    }
}
