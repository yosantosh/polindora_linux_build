import QtQuick
import ".."

Rectangle {
    id: root
    property string label: ""
    property bool highlighted: false
    signal clicked()

    width: 42
    height: 42
    radius: width / 2
    scale: pointer.pressed ? 0.94 : pointer.containsMouse ? 1.06 : 1.0
    color: highlighted ? Qt.rgba(0.25, 0.25, 0.9, 0.68) : Qt.rgba(0.16, 0.22, 0.36, 0.76)
    border.color: highlighted ? Theme.accent2 : Qt.rgba(0.62, 0.72, 0.94, 0.35)
    border.width: 1

    Behavior on scale {
        NumberAnimation { duration: Theme.animationFast; easing.type: Easing.OutBack }
    }

    Text {
        anchors.centerIn: parent
        text: root.label
        color: Theme.textPrimary
        font.pixelSize: 24
        font.weight: Font.Light
    }

    MouseArea {
        id: pointer
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: root.clicked()
    }
}
