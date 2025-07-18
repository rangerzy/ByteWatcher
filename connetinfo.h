#ifndef CONNETINFO_H
#define CONNETINFO_H

#include <QObject>
#include <QQmlEngine>
#include <serial.h>
#include <QThread>
#include <QRunnable>

class ConnetInfo : public QObject
{
    Q_OBJECT

    Q_PROPERTY(QList<QString> ports READ ports WRITE setPorts NOTIFY portsChanged FINAL)
public:
    ConnetInfo(QObject* parent = nullptr): QObject(parent){};

    QList<QString> ports() const;
    void setPorts(const QList<QString> &newPorts);

    Q_INVOKABLE void getPorts();

    Q_INVOKABLE void connectPort(QString path, quint32 buad, quint8 data_bit, quint8 stop_bit);
    Q_INVOKABLE void disconnectPort();

signals:
    void portsChanged();
    void dataChanged(QString msg);

private:
    QList<QString> m_ports;
    QThread *m_monitorThread = nullptr;
    QRunnable *connectThread = nullptr;
    serial::Serial *connetct_serial = nullptr;
};

#endif // CONNETINFO_H
