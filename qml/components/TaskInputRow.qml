import QtQuick
import QtQuick.Layouts
import ".."

GlassPanel {
    id: root
    property alias text: input.text
    signal accepted(string title)

    height: 62

    RowLayout {
        anchors.fill: parent
        anchors.margins: 14
        spacing: 12

        TextInput {
            id: input
            Layout.fillWidth: true
            color: Theme.textPrimary
            selectionColor: Theme.accent2
            selectedTextColor: Theme.textPrimary
            font.pixelSize: 15
            clip: true
            focus: false
            verticalAlignment: TextInput.AlignVCenter
            onAccepted: {
                root.accepted(text)
                text = ""
            }

            Text {
                anchors.verticalCenter: parent.verticalCenter
                text: "New focus task"
                color: Theme.textSecondary
                font.pixelSize: 15
                visible: input.text.length === 0
            }
        }

        GlassButton {
            Layout.preferredWidth: 64
            Layout.fillHeight: true
            label: "Add"
            highlighted: true
            onClicked: {
                root.accepted(input.text)
                input.text = ""
            }
        }
    }
}
