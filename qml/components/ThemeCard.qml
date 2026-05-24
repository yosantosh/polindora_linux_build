import QtQuick
import QtQuick.Layouts
import ".."

GlassPanel {
    id: root
    property string name: ""
    property string description: ""
    property string accentColor: Theme.accent
    property bool selected: false
    signal clicked()

    height: 104
    border.color: selected ? accentColor : Qt.rgba(0.55, 0.68, 0.95, 0.32)

    RowLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 14

        Rectangle {
            Layout.preferredWidth: 62
            Layout.preferredHeight: 62
            radius: 18
            color: "#071329"
            border.color: root.accentColor
            border.width: 1

            Rectangle {
                width: 34
                height: 34
                radius: 17
                anchors.centerIn: parent
                color: root.accentColor
                opacity: 0.72
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 5

            Text {
                text: root.name
                color: Theme.textPrimary
                font.pixelSize: 16
                font.weight: Font.DemiBold
            }

            Text {
                Layout.fillWidth: true
                text: root.description
                color: Theme.textSecondary
                font.pixelSize: 12
                wrapMode: Text.WordWrap
            }
        }

        Text {
            text: selected ? "Selected" : "Use"
            color: selected ? root.accentColor : Theme.textSecondary
            font.pixelSize: 12
            font.weight: Font.Medium
        }
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        cursorShape: Qt.PointingHandCursor
        onClicked: root.clicked()
    }
}
