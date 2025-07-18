import QtQuick
import QtQuick.Layouts
import QtQuick.Window
import QtQuick.Controls

Item {
    property int row_height: 35

    Rectangle {
        width: parent.width
        height: parent.height
        opacity: 1
        border.color: "#f0f0f0"
        border.width: 1
        radius: 2

        Flow {
            width: parent.width
            height: parent.height
            // 串口信息
            RowLayout {
                height: row_height
                width: parent.width
                Item {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Layout.leftMargin: 5
                    Layout.rightMargin: 5
                    Text {
                        anchors.verticalCenter: parent.verticalCenter
                        anchors.left: parent.left
                        width: parent.width *0.5
                        text: "串口1"
                        font.pixelSize: 16
                    }
                    ComboBox {
                        id: serial_path
                        anchors.verticalCenter: parent.verticalCenter
                        editable: false
                        anchors.right: parent.right
                        width: parent.width *0.5
                        model: ListModel {
                            id: serials
                        }
                        currentIndex: 0
                        font.pixelSize: 16
                        Component.onCompleted: {
                            console.log("开始获取串口信息")
                            connet_info.getPorts()
                        }
                    }
                }
            }
            // 波特率
            RowLayout {
                height: row_height
                width: parent.width
                Item {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Layout.leftMargin: 5
                    Layout.rightMargin: 5
                    Text {
                        anchors.verticalCenter: parent.verticalCenter
                        anchors.left: parent.left
                        width: parent.width *0.5
                        text: "波特率"
                        font.pixelSize: 16
                    }
                    ComboBox {
                        id: serial_baud
                        anchors.verticalCenter: parent.verticalCenter
                        editable: true
                        anchors.right: parent.right
                        width: parent.width *0.5
                        model: ListModel {
                            ListElement {text: "4800"}
                            ListElement {text: "9600"}
                            ListElement {text: "14400"}
                            ListElement {text: "19200"}
                            ListElement {text: "38400"}
                            ListElement {text: "56000"}
                            ListElement {text: "57600"}
                            ListElement {text: "115200"}
                            ListElement {text: "128000"}
                            ListElement {text: "230400"}
                            ListElement {text: "1000000"}
                        }
                        currentIndex: 7
                        font.pixelSize: 16
                        onAccepted: {
                            if (find(editText) === -1)
                                model.append({text: editText})
                        }
                    }
                }
            }
            // 数据位
            RowLayout {
                height: row_height
                width: parent.width
                Item {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Layout.leftMargin: 5
                    Layout.rightMargin: 5
                    Text {
                        anchors.verticalCenter: parent.verticalCenter
                        anchors.left: parent.left
                        width: parent.width *0.5
                        text: "数据位"
                        font.pixelSize:16
                    }
                    TextField {
                        id: serial_data_bit
                        anchors.verticalCenter: parent.verticalCenter
                        anchors.right: parent.right
                        width: parent.width *0.5
                        font.pixelSize: 16
                        validator: IntValidator {
                            bottom: 0
                            top: 99
                        }
                        text: "8"
                        inputMethodHints: Qt.ImhDigitsOnly
                    }
                }
            }
            // 停止位
            RowLayout {
                height: row_height
                width: parent.width
                Item {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Layout.leftMargin: 5
                    Layout.rightMargin: 5
                    Text {
                        anchors.verticalCenter: parent.verticalCenter
                        anchors.left: parent.left
                        width: parent.width *0.5
                        text: "停止位"
                        font.pixelSize:16
                    }
                    TextField {
                        id: serial_stop_bit
                        anchors.verticalCenter: parent.verticalCenter
                        anchors.right: parent.right
                        width: parent.width *0.5
                        font.pixelSize: 16
                        validator: DoubleValidator {
                            bottom: 0.0
                            decimals:1
                            top: 99.9
                        }
                        text: "1"
                        inputMethodHints: Qt.ImhDigitsOnly
                    }
                }
            }
            // 连接按钮
            RowLayout {
                height: row_height
                width: parent.width
                Item {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Layout.leftMargin: 5
                    Layout.rightMargin: 5
                    Button {
                        anchors.verticalCenter: parent.verticalCenter
                        anchors.left: parent.left
                        width: parent.width
                        id: serial_connect

                        text: "连接"
                        font.pixelSize: 16
                        Rectangle {
                            id: serial_connect_bg
                            anchors.fill: serial_connect
                            color: "#f0f0f0"
                            opacity: enabled ? 1 : 0.3
                            // border.color: control.down ? "#17a81a" : "#21be2b"
                            border.width: 1
                            radius: 2
                        }

                        onClicked: {
                            serial_connect.enabled = false
                            if (serial_connect.text === "连接") {
                                connet_info.connectPort(serial_path.currentText, serial_baud.currentText, serial_data_bit.text, serial_stop_bit.text)
                                console.log("2222222222222")
                                serial_connect.text = "断开"
                                serial_connect.palette.buttonText=Qt.rgba(1,1,1,1)
                                serial_connect_bg.color = Qt.rgba(0xe9/255,0x1e/255,0x63/255,1)
                            } else {
                                connet_info.disconnectPort()
                                serial_connect.text = "连接"
                                serial_connect.palette.buttonText=Qt.rgba(0,0,0,1)
                                serial_connect_bg.color = "#f0f0f0"
                            }
                            serial_connect.enabled = true
                        }
                    }
                }
            }
        }

        Connections {
            target: connet_info
            function onPortsChanged() {
                serials.clear()
                for (const p of connet_info.ports) {
                    serials.append({text: p})
                }
            }
        }
    }
}
