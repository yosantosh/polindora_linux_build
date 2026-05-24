import QtQuick
import ".."

Rectangle {
    id: root
    radius: Theme.radiusMedium
    color: Qt.rgba(0.09, 0.16, 0.29, 0.62)
    border.color: Qt.rgba(0.55, 0.68, 0.95, 0.42)
    border.width: 1

    Rectangle {
        anchors.fill: parent
        radius: parent.radius
        opacity: 0.42
        gradient: Gradient {
            GradientStop { position: 0.0; color: Qt.rgba(1, 1, 1, 0.14) }
            GradientStop { position: 0.45; color: Qt.rgba(1, 1, 1, 0.03) }
            GradientStop { position: 1.0; color: Qt.rgba(0.1, 0.35, 0.9, 0.12) }
        }
    }
}
