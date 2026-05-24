import QtQuick
import QtQuick.Layouts
import ".."

GlassPanel {
    id: root
    property string currentTab: "timer"
    signal tabSelected(string tab)

    height: 76
    radius: 22

    RowLayout {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 8

        Repeater {
            model: [
                { key: "timer", icon: "o", label: "Timer" },
                { key: "behavior", icon: "*", label: "Behavior" },
                { key: "appearance", icon: "@", label: "Appearance" },
                { key: "analytics", icon: "|", label: "Analytics" },
                { key: "tasks", icon: "=", label: "Tasks" },
                { key: "theme", icon: "~", label: "Theme" }
            ]

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: Theme.radiusSmall
                color: root.currentTab === modelData.key
                    ? Qt.rgba(0.28, 0.28, 0.88, 0.56)
                    : Qt.rgba(0.1, 0.16, 0.28, 0.46)
                border.color: root.currentTab === modelData.key ? Theme.accent2 : "transparent"
                border.width: 1
                scale: pointer.pressed ? 0.96 : pointer.containsMouse ? 1.03 : 1.0

                Behavior on scale {
                    NumberAnimation { duration: Theme.animationFast; easing.type: Easing.OutBack }
                }

                ColumnLayout {
                    anchors.centerIn: parent
                    spacing: 3

                    Text {
                        Layout.alignment: Qt.AlignHCenter
                        text: modelData.icon
                        color: Theme.textPrimary
                        font.pixelSize: 17
                    }

                    Text {
                        Layout.alignment: Qt.AlignHCenter
                        text: modelData.label
                        color: Theme.textPrimary
                        font.pixelSize: 11
                    }
                }

                MouseArea {
                    id: pointer
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: root.tabSelected(modelData.key)
                }
            }
        }
    }
}
