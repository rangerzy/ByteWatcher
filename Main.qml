import QtQuick
import QtQuick.Controls
import ByteDealer 1.0
ApplicationWindow {
    x: 1950
    y: 100
    width: 980
    height: 480
    visible: true
    title: qsTr("ByteDealer")
    flags: Qt.Window |
           Qt.WindowStaysOnTopHint |
           Qt.CustomizeWindowHint |
           Qt.WindowTitleHint |
           Qt.WindowMinMaxButtonsHint |
           Qt.WindowCloseButtonHint
    ConnetInfo {
        id:connet_info
    }

    ConnectView {
        id:connect_view
        width: Math.max(200, parent.width*0.2)
        height: parent.height
        // anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
    }

    DataView {
        x:connect_view.width
        width: parent.width - connect_view.width
        height: parent.height
        // anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
    }
}
