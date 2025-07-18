#include "connetinfo.h"

#include <QGuiApplication>
#include <QQmlApplicationEngine>



int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);


    qmlRegisterType<ConnetInfo>("ByteDealer", 1, 0, "ConnetInfo");

    // qmlRegisterSingletonType<ConnetInfo>(
    //     "ConnetInfo",
    //     1,
    //     0,
    //     "ConnetInfo",
    //     [](QQmlEngine* engine, QJSEngine* scriptEngine) -> QObject* {
    //         Q_UNUSED(engine); Q_UNUSED(scriptEngine);
    //         return new ConnetInfo; // 仅首次调用时实例化
    //     });

    const QUrl url(QStringLiteral("qrc:/Main.qml"));
    QQmlApplicationEngine engine;
    QObject::connect(
        &engine,
        &QQmlApplicationEngine::objectCreationFailed,
        &app,
        []() { QCoreApplication::exit(-1); },
        Qt::QueuedConnection);

    engine.load(url);

    return app.exec();
}
