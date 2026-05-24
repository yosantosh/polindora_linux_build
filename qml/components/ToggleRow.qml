import QtQuick
import QtQuick.Layouts
import ".."

GlassPanel {
    id: root
    property string iconText: ""
    property string title: ""
    property string subtitle: ""
    property bool checked: false
    signal toggled()

    height: 82
    radius: Theme.radiusMedium

    RowLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 16

        Rectangle {
            Layout.preferredWidth: 46
            Layout.preferredHeight: 46
            radius: 23
            color: checked ? Qt.rgba(0.23, 0.32, 0.85, 0.54) : Qt.rgba(0.14, 0.2, 0.34, 0.66)
            border.color: checked ? Theme.accent2 : Theme.accent
            border.width: 1

            Text {
                anchors.centerIn: parent
                text: root.iconText
                color: Theme.textPrimary
                font.pixelSize: 18
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 3

            Text {
                text: root.title
                color: Theme.textPrimary
                font.pixelSize: 16
                font.weight: Font.Medium
            }

            Text {
                text: root.subtitle
                color: Theme.textSecondary
                font.pixelSize: 12
                wrapMode: Text.WordWrap
            }
        }

        Rectangle {
            Layout.preferredWidth: 54
            Layout.preferredHeight: 30
            radius: 15
            color: checked ? Qt.rgba(0.3, 0.42, 1.0, 0.78) : Qt.rgba(0.12, 0.16, 0.27, 0.88)
            border.color: checked ? Theme.accent2 : Qt.rgba(0.6, 0.7, 0.9, 0.28)
            border.width: 1

            Rectangle {
                width: 24
                height: 24
                radius: 12
                y: 3
                x: checked ? parent.width - width - 3 : 3
                color: Theme.textPrimary

                Behavior on x {
                    NumberAnimation {
                        duration: Theme.animationMedium
                        easing.type: Easing.OutBack
                    }
                }
            }
        }
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: root.toggled()
    }
}
