import QtQuick
import QtQuick.Controls
import ByteDealer 1.0

Item {

    property int sum : 0
    ListModel {
        id: logModel
    }
    function addPortToModel(portName) {
        logModel.append({
            time: Qt.formatDateTime(new Date(), "yyyy-MM-dd HH:mm:ss.zzz"),
            message: "aa1b00eb020000289802050000080110a31a701700011075ea0155aa1b00ec02000005c92102000008101000002722b4ffa4ffea0355aa1b00ed020000b1bf220200000810102976000000000000eb0055aa1b00ee0200006d24090500000801103502013406013401cf0055aa1b00ef020000d952260200000810100300ffff010067ffb80355aa1b00f0020000909d2702000008101060002722a0ff4800e10255aa1b00f102000024eb28020000081010f2ff10001a0030fc990355aa1b00f2020000f87029020000081010303a61dd2976feb24a0455aa1b00f30200004c062a0200000810101b1a51533bfa372dc60255aa1b00f402000061572c0200000810100001060000014a01a90055aa1b00f5020000d5212d02000008101004040000000000005f0055aa1b00f602000009ba110500000801100000000000010000300055aa1b00f7020000bdcc2e0200000810100000000000000000580055aa1b00f802000053182f020000081010e0b100ff00000000e90255aa1b00f9020000e76e0d05000008011009343534370000020a0155aa1b00fa0200003bf53002000008101000000000000000005a0055aa1b00fb0200008f833102000008101000000000010000005c0055aa1b00fc020000a2d243020000081010731a515391fa372d8d0355aa1b00fd02000016a444020000081010f8b2000000000100190255aa1b00fe020000ca3f16050000080110e803000001000000200155aa1b00ff0200007e4945020000081010000019493f2205255c0155aa1b0000020000dd02460200000810103200000000000000a20055aa1b00010200006974470200000810102c1a5153e8fb372da20355aa1b0002020000b5ef4802000008101002b3fcff06000d00350355aa1b00030200000199490200000810100700a5140000031a500155aa1b00040200002cc805050000080110e803f003e8030000ec0255aa1b000502000098be4a0200000810103900030000000000b00055aa1b000602000044250c0200000801100000000000000000270055aa1b0007020000f0534b0200000810100000000000000000750055aa1b00080200001e870a050000080110421032106203 " + sum
        });
        sum ++
        // 限制日志数量（性能关键）
        if(logModel.count > 500) {
            logModel.clear();
        }
    }
    Timer {
        id: timer
        interval: 50
        repeat: true
        running: false
        onTriggered: addPortToModel(new Date().toLocaleString())
    }
    Rectangle {
        anchors.leftMargin: 10
        width: parent.width
        height: parent.height
        opacity: 1
        border.color: "#f0f0f0"
        border.width: 2
        radius: 2

        ListView {
            width: parent.width
            height: parent.height
            clip: true
            // spacing: 2
            // 性能优化配置
            cacheBuffer: height * 2  // 预缓存区域
            reuseItems: true         // 启用项复用
            flickDeceleration: 2000  // 平滑滚动
            model: logModel
            delegate:Text {
                text: "["+time+"]" + message
                wrapMode: Text.Wrap
                color: "#000000"
                font.pixelSize: 14
                maximumLineCount: 5  // 可选限制最大行数
                elide: Text.ElideRight  // 超出部分显示省略号
            }
            onCountChanged: positionViewAtEnd()
            ScrollBar.vertical: ScrollBar {
                width: 10
                active: true
            }
        }
    }


    Connections {
        target: connet_info
        function onDataChanged(msg) {
            logModel.append({
                time: Qt.formatDateTime(new Date(), "yyyy-MM-dd HH:mm:ss.zzz"),
                message: msg
            });
            // 限制日志数量（性能关键）
            if(logModel.count > 500) {
                logModel.clear();
            }
        }
    }

}
