// main.qml placeholder 
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs // For FileDialog
import com.daxa.gui 1.0 // Import our Rust QObject

ApplicationWindow {
    id: root
    visible: true
    width: 800
    height: 600
    title: "Daxa File Explorer"

    // Instantiate the Rust QObject
    DaxaViewModel {
        id: viewModel
    }

    FileDialog {
        id: fileDialog
        title: "Open Daxa File"
        nameFilters: ["Daxa files (*.dax *.daxa *.daxm *.daxlog)", "All files (*)"]
        currentFilePath: viewModel.get_file_path_for_dialog() // Use invokable for path
        onAccepted: {
            console.log("Selected file: " + selectedFile)
            // QML returns URL, convert to local file path string for Rust
            var localPath = selectedFile.toString()
            if (localPath.startsWith("file:///")) {
                 // Handle Windows paths starting with file:///C:/...
                if (Qt.platform.os === "windows" && localPath.charAt(8) === ":") {
                    localPath = localPath.substring(8) // Remove file:///
                } else {
                    localPath = localPath.substring(7) // Remove file://
                }
            }
            viewModel.load_daxa_file(localPath)
        }
    }

    header: ToolBar {
        RowLayout {
            anchors.fill: parent
            ToolButton {
                text: qsTr("Open File...")
                onClicked: fileDialog.open()
            }
            Label {
                id: currentFileLabel
                text: viewModel.filePath ? "File: " + viewModel.filePath : "No file loaded"
                Layout.fillWidth: true
                elide: Text.ElideRight
            }
        }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 10

        Label {
            text: "Daxa File Content"
            font.bold: true
            font.pixelSize: 18
        }

        Label {
            id: statusLabel
            text: viewModel.statusMessage
            color: viewModel.isFileLoaded ? "green" : "red"
            Layout.fillWidth: true
            wrapMode: Text.Wrap
        }

        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            ColumnLayout {
                width: parent.width

                GroupBox {
                    title: "Schema Overview"
                    Layout.fillWidth: true
                    Label {
                        text: viewModel.schemaSummary
                        wrapMode: Text.Wrap
                    }
                    // TODO: Add a ListView here for schema types/enums using a model from viewModel
                }

                GroupBox {
                    title: "Data Overview"
                    Layout.fillWidth: true
                    Label {
                        text: viewModel.dataSummary
                        wrapMode: Text.Wrap
                    }
                    // TODO: Add a ListView or TreeView for data items
                }

                // Placeholder for Viewer.qml or Explorer.qml
                // Viewer {
                //     Layout.fillWidth: true
                //     // Pass data from viewModel
                // }
            }
        }
    }

    footer: StatusBar {
        Label {
            text: viewModel.is_file_loaded ? "File Loaded" : "Ready"
        }
    }
}