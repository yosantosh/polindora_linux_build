import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import com.polindora.app
import "components"

Window {
    id: window
    width: 496
    height: 793
    visible: true
    color: "transparent"
    title: "Polindora"
    property bool settingsOpen: false

    PolindoraController {
        id: backend
    }

    Rectangle {
        anchors.fill: parent
        radius: 28
        color: Theme.background0
        border.color: Qt.rgba(0.43, 0.6, 0.9, 0.72)
        border.width: 1
        clip: true

        Rectangle {
            anchors.fill: parent
            gradient: Gradient {
                GradientStop { position: 0.0; color: Theme.background1 }
                GradientStop { position: 0.45; color: Theme.background0 }
                GradientStop { position: 1.0; color: "#001b3d" }
            }
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 28
            spacing: 18
            visible: !window.settingsOpen

            RowLayout {
                Layout.fillWidth: true
                Layout.preferredHeight: 48

                Item { Layout.fillWidth: true }

                Text {
                    text: "Polindora"
                    color: Theme.textPrimary
                    font.pixelSize: 22
                    font.weight: Font.DemiBold
                }

                Item { Layout.fillWidth: true }

                GlassButton {
                    Layout.preferredWidth: 38
                    Layout.preferredHeight: 38
                    label: "*"
                    onClicked: window.settingsOpen = true
                }
            }

            TimerOrb {
                id: timerOrb
                Layout.alignment: Qt.AlignHCenter
                Layout.preferredWidth: 328
                Layout.preferredHeight: 328
                modeText: "FOCUS"
                timeText: formatSeconds(backend.remainingSeconds)
                progress: backend.progress
            }

            Text {
                Layout.fillWidth: true
                Layout.leftMargin: 44
                Layout.rightMargin: 44
                text: backend.quoteText
                color: Theme.textSecondary
                wrapMode: Text.WordWrap
                horizontalAlignment: Text.AlignHCenter
                lineHeight: 1.25
                font.pixelSize: 16
                font.italic: true
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 16

                GlassButton {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 64
                    label: backend.activeCategory
                }

                GlassButton {
                    Layout.preferredWidth: 86
                    Layout.preferredHeight: 64
                    label: backend.isRunning ? "Pause" : backend.isPaused ? "Resume" : "Start"
                    highlighted: true
                    onClicked: {
                        if (backend.isRunning) {
                            backend.pauseTimer()
                        } else if (backend.isPaused) {
                            backend.resumeTimer()
                        } else {
                            backend.startWork()
                        }
                    }
                }

                GlassButton {
                    Layout.preferredWidth: 72
                    Layout.preferredHeight: 64
                    label: "Skip"
                    visible: !(backend.strictMode && backend.phase === "work")
                    onClicked: backend.skipTimer()
                }

                GlassButton {
                    Layout.preferredWidth: 72
                    Layout.preferredHeight: 64
                    label: "Reset"
                    onClicked: backend.resetTimer()
                }
            }

            SectionTitle {
                title: "Today"
            }

            GlassPanel {
                Layout.fillWidth: true
                Layout.preferredHeight: 62

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 16
                    spacing: 14

                    CircleBadge { label: "o" }

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 0
                        Text {
                            text: backend.sessionsToday + " pomodoros"
                            color: Theme.textPrimary
                            font.pixelSize: 16
                        }
                        Text {
                            text: backend.focusMinutesToday + "m focused"
                            color: Theme.textSecondary
                            font.pixelSize: 13
                        }
                    }

                    Text {
                        text: ">"
                        color: Theme.textPrimary
                        font.pixelSize: 22
                    }
                }
            }

            SectionTitle {
                title: "Top Tasks"
            }

            GlassPanel {
                Layout.fillWidth: true
                Layout.preferredHeight: 62

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 16
                    spacing: 14

                    CircleBadge { label: "[]"}

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 0
                        Text {
                            text: backend.taskActiveCount > 0 ? backend.taskActiveCount + " active tasks" : "No active tasks"
                            color: Theme.textPrimary
                            font.pixelSize: 16
                        }
                        Text {
                            text: backend.taskActiveCount > 0 ? "Open Tasks to review focus queue" : "Add tasks from the Tasks tab"
                            color: Theme.textSecondary
                            font.pixelSize: 13
                        }
                    }

                    Text {
                        text: ">"
                        color: Theme.textPrimary
                        font.pixelSize: 22
                    }
                }
            }
        }

        SettingsPanel {
            anchors.fill: parent
            backend: backend
            visible: window.settingsOpen
            onCloseRequested: window.settingsOpen = false
        }
    }

    component SectionTitle: RowLayout {
        property string title
        Layout.fillWidth: true
        spacing: 8

        Rectangle {
            Layout.preferredWidth: 2
            Layout.preferredHeight: 16
            radius: 2
            color: Theme.accent
        }

        Text {
            text: title
            color: Theme.textPrimary
            font.pixelSize: 16
            font.weight: Font.DemiBold
        }
    }

    component CircleBadge: Rectangle {
        property string label
        Layout.preferredWidth: 42
        Layout.preferredHeight: 42
        radius: 21
        color: "#123b82"
        border.color: Theme.accent
        border.width: 1

        Text {
            anchors.centerIn: parent
            text: label
            color: Theme.textPrimary
            font.pixelSize: 16
            font.weight: Font.DemiBold
        }
    }

    function formatSeconds(totalSeconds) {
        var safeSeconds = Math.max(0, totalSeconds)
        var minutes = Math.floor(safeSeconds / 60)
        var seconds = safeSeconds % 60
        return minutes + ":" + (seconds < 10 ? "0" + seconds : seconds)
    }
}
