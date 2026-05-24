import QtQuick
import QtQuick.Layouts
import ".."

Rectangle {
    id: root
    property string label: ""
    property bool highlighted: false
    signal clicked()

    radius: Theme.radiusMedium
    scale: pointer.pressed ? 0.96 : pointer.containsMouse ? 1.03 : 1.0
    color: highlighted ? Qt.rgba(0.19, 0.22, 0.92, 0.72) : Qt.rgba(0.11, 0.17, 0.29, 0.72)
    border.color: highlighted ? Theme.accent2 : Qt.rgba(0.66, 0.76, 0.95, 0.45)
    border.width: 1

    Behavior on scale {
        NumberAnimation {
            duration: Theme.animationFast
            easing.type: Easing.OutBack
        }
    }

    Behavior on color {
        ColorAnimation { duration: Theme.animationMedium }
    }

    Text {
        anchors.centerIn: parent
        text: root.label
        color: Theme.textPrimary
        font.pixelSize: 15
        font.weight: root.highlighted ? Font.DemiBold : Font.Medium
    }

    MouseArea {
        id: pointer
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: root.clicked()
    }
}
