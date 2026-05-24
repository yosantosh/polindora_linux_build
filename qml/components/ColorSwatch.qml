import QtQuick
import ".."

Rectangle {
    id: root
    property string swatchColor: Theme.accent
    property bool selected: false
    signal clicked()

    width: 42
    height: 42
    radius: 21
    color: swatchColor
    border.color: selected ? Theme.textPrimary : Qt.rgba(1, 1, 1, 0.34)
    border.width: selected ? 3 : 1
    scale: pointer.pressed ? 0.92 : pointer.containsMouse ? 1.08 : 1.0

    Behavior on scale {
        NumberAnimation { duration: Theme.animationFast; easing.type: Easing.OutBack }
    }

    MouseArea {
        id: pointer
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: root.clicked()
    }
}
