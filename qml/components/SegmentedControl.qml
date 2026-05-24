import QtQuick
import QtQuick.Layouts
import ".."

GlassPanel {
    id: root
    property var options: ["Daily", "Weekly", "Monthly"]
    property int currentIndex: 0
    signal selected(int index)

    height: 44
    radius: Theme.radiusSmall

    RowLayout {
        anchors.fill: parent
        anchors.margins: 5
        spacing: 5

        Repeater {
            model: root.options

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: Theme.radiusSmall - 4
                color: root.currentIndex === index
                    ? Qt.rgba(0.28, 0.34, 0.92, 0.68)
                    : Qt.rgba(0.08, 0.13, 0.24, 0.2)
                border.color: root.currentIndex === index ? Theme.accent2 : "transparent"
                border.width: 1
                scale: pointer.pressed ? 0.97 : pointer.containsMouse ? 1.02 : 1.0

                Behavior on scale {
                    NumberAnimation { duration: Theme.animationFast; easing.type: Easing.OutBack }
                }

                Text {
                    anchors.centerIn: parent
                    text: modelData
                    color: Theme.textPrimary
                    font.pixelSize: 12
                    font.weight: Font.Medium
                }

                MouseArea {
                    id: pointer
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        root.currentIndex = index
                        root.selected(index)
                    }
                }
            }
        }
    }
}
