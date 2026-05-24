import QtQuick
import QtQuick.Layouts
import ".."

Item {
    id: root
    property string modeText: "FOCUS"
    property string timeText: "25:00"
    property real progress: 0.0

    Rectangle {
        id: glow
        anchors.centerIn: parent
        width: parent.width * 0.9
        height: width
        radius: width / 2
        color: Qt.rgba(0.19, 0.46, 1.0, 0.16)
        scale: 1.0

        SequentialAnimation on opacity {
            loops: Animation.Infinite
            NumberAnimation { to: 0.38; duration: Theme.animationSlow; easing.type: Easing.InOutSine }
            NumberAnimation { to: 0.72; duration: Theme.animationSlow; easing.type: Easing.InOutSine }
        }
    }

    Rectangle {
        anchors.centerIn: parent
        width: parent.width * 0.78
        height: width
        radius: width / 2
        color: Qt.rgba(0.06, 0.12, 0.24, 0.82)
        border.color: Qt.rgba(0.72, 0.8, 1.0, 0.86)
        border.width: 1

        gradient: Gradient {
            GradientStop { position: 0.0; color: Qt.rgba(1, 1, 1, 0.3) }
            GradientStop { position: 0.32; color: Qt.rgba(0.1, 0.18, 0.36, 0.84) }
            GradientStop { position: 1.0; color: Qt.rgba(0.0, 0.04, 0.13, 0.94) }
        }
    }

    ProgressRing {
        anchors.fill: parent
        progress: root.progress
        lineWidth: 7
    }

    ColumnLayout {
        anchors.centerIn: parent
        spacing: 16

        Text {
            Layout.alignment: Qt.AlignHCenter
            text: "v"
            color: Theme.accent
            font.pixelSize: 26
            font.weight: Font.Bold
        }

        Text {
            Layout.alignment: Qt.AlignHCenter
            text: root.modeText
            color: Theme.textSecondary
            font.pixelSize: 15
        }

        Text {
            Layout.alignment: Qt.AlignHCenter
            text: root.timeText
            color: Theme.textPrimary
            font.pixelSize: 56
            font.weight: Font.Light
        }
    }
}
