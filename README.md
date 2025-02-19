**ProcMaster**

**Report**

Nour Abdalla 900213024, Sarah Elsamanody 900212915, Hussien Heggi 900213220, Ali ElKhouly 900212679

The American University In Cairo

Dr. Amr ElKadi

**Table of contents**

1. [**Introduction:.............................................................................................................................................3**](#_page2_x72.00_y113.40)
1. [**Architecture Implementation:.................................................................................................................4**](#_page3_x72.00_y72.00)
1. [Back-end:................................................................................................................................... 4](#_page3_x72.00_y106.50)
1. [Front-end:...................................................................................................................................7](#_page6_x72.00_y237.59)
1. [TUI..........................................................................................................................................7](#_page6_x72.00_y267.48)
1. [GUI..........................................................................................................................................8](#_page7_x72.00_y237.59)
1. [**Features:..................................................................................................................................................10**](#_page9_x72.00_y72.00)
1. [Monitoring:............................................................................................................................... 10](#_page9_x72.00_y106.50)
1. [Controlling................................................................................................................................ 10](#_page9_x72.00_y247.21)
1. [UI/GUI...................................................................................................................................... 10](#_page9_x72.00_y443.32)
1. [**Integration:............................................................................................................................................. 11**](#_page10_x72.00_y72.00)
1. [**How To Use:............................................................................................................................................12**](#_page11_x72.00_y72.00)
1. [Dependencies........................................................................................................................... 12](#_page11_x72.00_y106.50)
1. [Installation................................................................................................................................12](#_page11_x72.00_y277.83)
1. [How to Use?.............................................................................................................................12](#_page11_x72.00_y503.29)
1. [GUI........................................................................................................................................12](#_page11_x72.00_y533.18)
1. [TUI........................................................................................................................................13](#_page12_x72.00_y72.00)
1. [**Development Cycle:............................................................................................................................... 13**](#_page12_x72.00_y237.59)
1. [Planning and Design................................................................................................................ 13](#_page12_x72.00_y272.08)
1. [Implementation Process...........................................................................................................14](#_page13_x72.00_y72.00)
1. [Testing:.....................................................................................................................................19](#_page18_x72.00_y320.38)

   4. [Proof of Concept:......................................................................................................................20](#_page19_x72.00_y219.08)

1. [GUI.........................................................................................................................................20](#_page19_x72.00_y248.98)
1. [TUI:........................................................................................................................................23](#_page22_x72.00_y366.09)
1. [**Conclusion:............................................................................................................................................. 26**](#_page26_x72.00_y72.00)
1. [**Task Division:......................................................................................................................................... 27**](#_page27_x72.00_y72.00)
1. [TUI...........................................................................................................................................27](#_page27_x72.00_y106.50)
1. [GUI...........................................................................................................................................27](#_page27_x72.00_y301.98)
1. **Introduction:**

   <a name="_page2_x72.00_y113.40"></a>This report provides a detailed manual for our Linux Task Manager “**_ProcMaster_**”, an open-source application available in two versions: a Command-Line Interface (CLI) using Terminal User Interface (TUI) and a Graphical User Interface (GUI) built with Tauri and ReacJSt. **_ProcMaster_** is designed to efficiently monitor and manage system processes, making it a versatile solution for Linux users with diverse preferences and needs.

   With the increasing complexity of modern operating systems, task management tools have become essential for overseeing system performance, identifying resource-intensive processes, and terminating unresponsive applications. While Linux offers powerful command-line utilities for these tasks, many users desire an intuitive and accessible GUI alternative. At the same time, command-line enthusiasts prefer lightweight tools that provide detailed insights without leaving the terminal.

   **_ProcMaster_** bridges this gap by providing both a robust CLI version for terminal-based users and a user-friendly GUI version for those who prefer graphical interfaces. This report outlines the development journey of ProcMaster, detailing how to use both versions of the application, their back-end and front-end implementation, and the design decisions made. It also highlights the frameworks utilized and the application of Agile principles in the development process.

1. **Architecture<a name="_page3_x72.00_y72.00"></a> Implementation:**
1. **Back-end:**

<a name="_page3_x72.00_y106.50"></a>Rust was chosen for its exceptional performance, security, and reliability. Known for its strict compiler and memory safety guarantees, Rust ensures the back-end of the Task Manager is both efficient and free from common runtime errors. Its concurrency model is ideal for handling the demands of real-time process management.

- **Used Crates:**
- **serde::Serialize**

  Provides functionality for serializing Rust data structures into formats such as JSON, useful for passing data between the back-end and front-end or saving settings.

- **std::thread**

  Enables multi-threading in Rust, allowing tasks to run concurrently, such as monitoring system processes in the background.

- **std::ti me::Duration**

  Represents a span of time, useful for implementing delays, timeouts, or measuring execution intervals in the application.

- **std::path::Path**

  Represents file and directory paths in a cross-platform manner, helping the application interact with the filesystem.

- **std::fs::File**

  Provides functions for creating, reading, writing, and managing files on the filesystem.

- **std::i o::{self, BufRead, Write}**

  Provides utilities for input/output operations, such as buffered reading and writing, essential for interacting with files and user input.

- **tauri ::command**

  Enables defining custom commands that connect the Rust back-end to the Tauri front-end, facilitating communication between the two layers.

- **procfs::process::all_processes**

  A function from the procfs crate that retrieves a list of all running processes on a Linux system by reading the /proc filesystem.

- **procfs::{ti cks_per_second, Upti me}**

  Functions from the procfs crate for retrieving system-level information, such as the number of clock ticks per second and the system uptime.

- **sysi nfo::{System, SystemExt, RefreshKind}**

  Provides system monitoring utilities, such as accessing CPU, memory, and process information, through the sysinfo crate.

- **users::get_user_by_ui d**

  Retrieves user account details based on the user ID, helping associate processes with their respective owners.

- **ni x::sys::si gnal ::{ki l l , Si gnal }**

  Allows sending signals (e.g., SIGKILL, SIGTERM) to processes, enabling the application to terminate or control processes.

- **ni x::uni std::Pi d**

  Represents process IDs, used in conjunction with signals to identify the processes to target.

- **std::process::{Command, Stdi o}**

  Provides functionality for spawning and managing external processes, such as executing shell commands or monitoring specific tasks.

- **std::sync::{Mutex, Arc}**

  Enables thread-safe data sharing and synchronization in multi-threaded environments, ensuring consistency when accessing shared resources.

- **Ratatui ::{buffer, l ayout, styl e, text, wi dgets}**

  Crate that contains widgets and designs for UI rendering and implements on terminals

- **std::col l ecti ons::HashMap**

  A package to so we could use hashmaps to sort processes into hierarchical order by hashing them to their PPID

- **crossterm::event::{sel f, Event, KeyCode, KeyEventKi nd}** Enables us to handle key press events so we could sort and manage the TUI.
- **l i bc::{setpri ori ty, PRIO_PROCESS}**

  This includes the system call required to change the Nice value (or priority) of a certain process. This includes the same permissions that is available for htop.

- **use std::cmp::Reverse;**

  Enable This was used to reverse the order when sorting the data in the processes table.

2. **Front-end:**
1. <a name="_page6_x72.00_y237.59"></a>**TUI**

   <a name="_page6_x72.00_y267.48"></a>**Ratatui**

   Ratatui is a crate/library in Rust that is forked from the famous tui crate. It is mainly used for terminal user interface (TUI) applications where it offers multiple and different types of widgets for TUI designs, such as tables and gauges. We used these two mainly to present our consumptions and processes.

   **TUI Features**

   There are multiple features in the TUI program. They include showing each detail of every function in the way of a table, with each column a certain information of every process. We believe that knowing every possible thing about processes is essential in task managers. This also shows from our surveys about task managers and we tried to mimic them. Moreover, in other tabs we could monitor each core in the processor and its consumption using almost the same algorithm used by htop. Another tab shows us the memory consumption and disk usage. Our last tab is a settings page where not only can we switch between tree and table view for the processes, but also choose what columns we want to appear on the table. We used gauges in any percentage data such as core consumption for better user interface view for users. There are also simple instructions in the footer for anyone that wants to control their program. It shows how to sort, kill a program, switch between tabs, etc.

1. **GUI**

   <a name="_page7_x72.00_y237.59"></a>**ReactJS**

   ReactJS with Vite was used to implement the GUI’s front-end development. That is because ReactJS has a component-based architecture, which allows for seamless UI development and state management. Vite complements ReactJS by enabling faster development with minimal configuration compared to traditional bundlers like Webpack. The file structure for the front end is simple as it only contains _.jsx_ and _.css_ files.

**JSX Files**

These files are for writing the **HTML** components that we are using to render in the GUI. In our case, we have 2 **_JSX_** files representing our two main tabs or components which are process and resource information. These two tab components are then used in the **_app.js_** file which combines all components together in the tab format.

**CSS Files**

The use of CSS files is in order to be able to style our components to match one theme. We used **_CSS_** files in order to separate code and have a more structured and clean version that is easily interpretable.

**GUI Features and Library Used**

You will find two tabs one for the process table and controls and the other is for resources monitoring for the CPU, memory, and swap. The way we did our tab sections was by using the MUI React library which allowed for the integration of not just tabs, but also graphs such as pie charts, bar charts, and even icons.

3. **Features:**
1. <a name="_page9_x72.00_y72.00"></a>**Monitoring:**

- <a name="_page9_x72.00_y106.50"></a>Viewing running processes and their corresponding information
- Disk Usage
- Memory and Swap memory usage
- CPU affinity

2. **Controlling**

- <a name="_page9_x72.00_y247.21"></a>Process Management
- Killing
- Terminating
- Suspending
- Resuming
- Changing Priority

3. **UI/GUI**

- <a name="_page9_x72.00_y443.32"></a>Sorting: the process table can be ordered based on any criteria. **(T)(G)**
- Filtering: the process table can be altered to view specific columns. **(T)**
- Real-Time Updates: The system updates information in real-time. **(T)(G)**
- Process Tree View: The display of processes in a hierarchical format. **(T)**
- Memory Monitoring: visual reporting of memory using charts. **(T)(G)**
- CPU Monitoring: visual reporting of CPU usage using charts. **(T)(G)**

**Key: (T):** TUI **(G):** GUI

4. **Integration:**

   <a name="_page10_x72.00_y72.00"></a>Tauri serves as the bridge between the ReactJS front-end and the Rust back-end, seamlessly integrating both layers. Setting up Tauri is straightforward, requiring only a few guided steps from its comprehensive documentation to prepare the repository. Once configured, Tauri simplifies deployment and provides a robust command system, enabling the front-end to efficiently and securely invoke Rust functions through an intuitive API.

   Commands can accept arguments and return values. They can also return errors and be async. The command starts with **#[tauri ::command]** and an invoke handler with the name of the function is added to the main rust file.

   **i nvoke_handl er(tauri ::generate_handler![command])**

   At the same time, the JS file contains an invoke request **i mport { i nvoke } from ' @tauri -apps/ api / core' i nvoke(' command ' )**

   The RUST back-end serializes the processes’ info into a vector. Then it gets used to run the process table, CPU usage is serialized in a vector of struct instances, and the memory is serialized into an instance of a struct.

5. **How<a name="_page11_x72.00_y72.00"></a> To Use:**
6. **Dependencies**

<a name="_page11_x72.00_y106.50"></a>Make sure that you have the following installed:

1. Tauri
1. Rust
1. Cargo
1. Nodejs
1. Npm
1. **Installation**

<a name="_page11_x72.00_y277.83"></a>By applying the following steps to run GUI:

1. **git clone [https:/ / gi thub. com/ SamanodyJr/ OS_Project. gi t**](https://github.com/SamanodyJr/OS_Project.git)\*\*
1. **cd OS_Project/ ProcMaster**
1. **npmi nstal l**
1. **npmrun tauri dev**

By applying the following steps to run TUI:

1. **git clone [https:/ / gi thub. com/ SamanodyJr/ OS_Project. gi t**](https://github.com/SamanodyJr/OS_Project.git)\*\*
1. **cd OS_Project**
1. **cargo run**
1. **How<a name="_page11_x72.00_y503.29"></a> to Use?**
1. **GUI**

<a name="_page11_x72.00_y533.18"></a>The GUI is straightforward starting with two tabs as seen in the screenshot below one for the processes and the other for CPU and memory resources.

2. **TUI**

<a name="_page12_x72.00_y72.00"></a>The TUI should also be simple and straightforward as you will see in tab format and everything is written below to be able to know how to move between tabs and which keyboard clicks to use to be able to navigate as seen in the screenshot below.

<a name="_page12_x72.00_y237.59"></a>**6. Development Cycle:**

1. **Planning<a name="_page12_x72.00_y272.08"></a> and Design**

Our development cycle abstracts from the waterfall development cycle. We started our process by first surveying and researching different types of task managers. We found the names of some task managers and divided them among us to search every feature in each one of them and compare them to decide on the design of our task manager. After reviewing multiple task managers, we started choosing the features we would implement based on how people viewed them and our personal preferences. However, there were some features and designs that we were not able to implement unfortunately, but this will be discussed later.

Furthermore, after deciding on the features, we started researching the framework we would use and how we could use the system calls. Before we started our research, we divided the features we wanted to develop among us so we could work on them more efficiently. Every one of us took a part of the features we wanted to implement and searched on how we could collect this data.

2. **Implementation<a name="_page13_x72.00_y72.00"></a> Process**

After researching the functions and finding them, we started working on the TUI first and also separated the tasks among us to increase efficiency. We divided the tasks exactly like the latter tasks. In other words, whoever built the functions to retrieve data about the processes would implement its page, etc.

However, after the long run and debugging, we wanted to start working on the GUI, so we separated into two teams, one to start working and designing the framework for the GUI, and the other team to continue working on the TUI and finish its features.

**The TUI implementation:**

1. **_Process Monitoring_**

   Our process table gets its data from the crate **procfs** that reads its data from the file /proc which contains a folder and file for every process running on the machine. From this crate, we get a lot of information about the processes such as the user who started the process and its PID. Moreover, we use the crate system to get data from the device as in total memory for example so we could calculate the memory consumption of every process in terms of percentage. Finally, CPU consumption is calculated using a similar approach to htop as it calculates the total time used by the process divided by the total time the core has been used.

2. **_Process Controls_**

   We implemented five controls as mentioned above. Utilizing the crate nix that uses **SIGNALS** to be able to allow us to kill, terminate, pause, and resume processes. In addition, we were thinking of creating a killall function, its implementation is in the code and it works but it is not integrated within the tui. This function basically terminates all processes running a specific program to properly kill all instances of it if needed. Lastly, we have set priority which was the trickiest of all to implement and is not done using crates as we wanted to allow for privilege control. Thus, we were running the command **“sudo reni ce pri ori ty pi d”** however during the tui execution we were not able to prompt the user to input so we had it as a user command instead of using the crate libc.

3. **_Memory Monitoring_**

   In memory, we used the crate **sysinfo** that gets information from the system as a whole, where we get our total main memory and swap memory of the device, with the used memories as well. However, we first have to set them into GBs, so we divide them multiple times by 1024 to get to the correct unit.

4. **_Disk Monitoring_**

   When it came to monitoring the disks and I/O operations, we read directly from the proc/mounts file to get the devices’ names. Also, we started reading from the proc/diskstats to get the total stats from the I/O read and write operations.

5. **_CPU Monitoring_**

   For monitoring the consumption of every core in the processor, the task manager reads from proc/stat to track every core and calculate its consumption by dividing the total time it has been used over the total time it has been available (the time it has been used + total time it has been idle). We do that for every core. We track the cores from the file proc/stat by the start of the line, where every line that starts with cpu followed by a number is a core with that number as its index.

6. **_Real-Time_**

   One of our first hardships was showing the updated data in real-time on our task manager, so for the TUI we had to run some processes to fetch the data in the background so the data keeps on being updating, and then we had to redraw the TUI on a certain amount of time to reflect our updated fetched data.

7. **_TUI_**

   We started our work on the TUI by building an app that holds all the necessary data, such as whether the task manager is running or not, also with which page we are currently viewing and to hold all the updated data. Moreover, we had to build every page and group them together as tabs so we could easily switch between them. Finally, we keep on redrawing and rendering the pages every 100 milliseconds to update the view of the newly received and updated data. Additional features such as sorting that you do by pressing the key for sorting and numbered 0-9, A-B corresponding to each column. Another thing is eliminating some of the columns and choosing which ones to show or hide. These features can be accessed by the settings tab.

8. **_Tree View_**

   The tree view in this implementation works by organizing and displaying processes in a hierarchical structure based on their parent-child relationships. We join every process in a hashmap depending on its parent process identification (PPID). We then call a smaller function recursively to join every process with its children in a hierarchical way. For every child process, it is indented to the right to show it is not a parent process.

**The GUI implementation:**

We first created three tabs to divide it the same as the tui division of processes, CPU, and memory but it felt more logical to have GUI two tabs only to have a more visually appealing program.

1. **_Process Monitoring:_**

   We printed the table at first, and we managed to make it real-time as well as allow for sorting and implementing it in the GUI by clicking on the columns and the show of arrows to indicate the sorting. We also allow for the control to occur by right-clicking on the process selected or hovered on and using its PID we control it. The set priority in the GUI is the privilege command, unlike the TUI where we ask for a password to be able to lower it or make it higher.

2. **_Resources Tab:_**

   The resources tab includes 5 visual representations. One bar chart for the CPU usage is color-coded purple when it is less than 20%, yellow when it is less than 50%, and red when it is above. Two pie charts for the memory visualization, one for the main memory usage and the other for the SWAP memory, where the purple indicates free space in each and the red indicates what is being used. Two bar charts for the disk usage one for time spent and the other for the operations.

3. **_Real-Time:_**

   As was the case in the CLI showing real-time data was a hardship, we opted for using interval which is something that calls the invoke every a certain amount of time in our case each passing second. The problem was that if the user was viewing a sorted version of the processes after the update it went back to the default setting erasing the sort. To fix this we stored the sorting state and re-apply it after each update to ensure a user-friendly experience.

4. **Testing:**

<a name="_page18_x72.00_y320.38"></a>To validate ProcMaster's functionality and accuracy, we conducted extensive testing across multiple dimensions:

**_Monitoring Results Validation_**

We compared the information extracted by ProcMaster with established task managers such as _htop_ and _GNOME System Monitor_. This comparative analysis ensured that ProcMaster accurately retrieves and displays process information.

**_Control Testing_**

The controls were rigorously tested by launching specific applications and tracing their behaviors within ProcMaster. This process allowed us to verify the correct execution of commands and the responsiveness of the control features.

**_CPU Calculations_**

CPU usage calculations were a key focus during testing, as
different task managers implement these calculations differently. Factors such as elapsed time and other metrics are often considered, leading to variations in reported values. Our implementation of CPU calculations aligned most closely with _htop_, confirming the accuracy and reliability of our approach.

**_Results_**

The testing phase demonstrated that **_ProcMaster_** operates effectively, with all equations and computations performing as intended. Users can trust ProcMaster to provide accurate monitoring and control functionality.

<a name="_page19_x72.00_y219.08"></a>**6.4 Proof of Concept:**

1. **GUI**

<a name="_page19_x72.00_y248.98"></a>The process table:

![](Aspose.Words.23d6e4e8-ae3b-4c98-87e5-44b114f2a6f8.001.jpeg)

Control through right click:

![](Aspose.Words.23d6e4e8-ae3b-4c98-87e5-44b114f2a6f8.002.jpeg)

Change Priority Tab:

![](Aspose.Words.23d6e4e8-ae3b-4c98-87e5-44b114f2a6f8.003.jpeg)

Resources CPU usage:

![](Aspose.Words.23d6e4e8-ae3b-4c98-87e5-44b114f2a6f8.004.jpeg)

Memory and Swap Usage:

![](Aspose.Words.23d6e4e8-ae3b-4c98-87e5-44b114f2a6f8.005.jpeg)

Disk Usage:

![](Aspose.Words.23d6e4e8-ae3b-4c98-87e5-44b114f2a6f8.006.jpeg)

2. **TUI:**

   <a name="_page22_x72.00_y366.09"></a>Process Table

![](Aspose.Words.23d6e4e8-ae3b-4c98-87e5-44b114f2a6f8.007.jpeg)

Controls (Pressing C)

![](Aspose.Words.23d6e4e8-ae3b-4c98-87e5-44b114f2a6f8.008.jpeg)

Sorting (Pressing W)

![](Aspose.Words.23d6e4e8-ae3b-4c98-87e5-44b114f2a6f8.009.jpeg)

Core Consumption

![](Aspose.Words.23d6e4e8-ae3b-4c98-87e5-44b114f2a6f8.010.jpeg)

Memory Usage, Disk & I/O operations.

![](Aspose.Words.23d6e4e8-ae3b-4c98-87e5-44b114f2a6f8.011.jpeg)

Settings page

![](Aspose.Words.23d6e4e8-ae3b-4c98-87e5-44b114f2a6f8.012.jpeg)

7. **Conclusion:**

<a name="_page26_x72.00_y72.00"></a>In this report, we have provided a comprehensive review of ProcMaster, examining its technical implementation and design aspects in detail. The goal was to offer potential users and stakeholders an in-depth understanding of the system's capabilities, architecture, and functionality. ProcMaster is designed to be a robust and user-friendly task manager, equipped with features that cater to the needs of both technical and non-technical users. By following the guidelines and explanations provided in this manual, users will be able to effectively leverage ProcMaster for managing processes, optimizing workflows, and improving overall system productivity. We believe that the insights presented here will help users maximize the value of ProcMaster, making it a reliable tool for diverse applications.

### **8. Task Division**

#### **8.1 TUI**

- **Process Monitoring**: Nour
- **Process Control**: Sarah
- **CPU Usage**: Ali
- **Memory & Disk Usage**: Hussein
- **Visuals**: Sarah & Ali
- **Real-Time, Tree View, and TUI extra features**: Ali

#### **8.2 GUI**

- **Process Monitoring & Sorting**: Sarah
- **Process Control, Disk Usage, and Real-Time**: Hussein
- **CPU Usage, Memory Usage & Real-Time**: Nour
- **Visuals**: Sarah, Hussein & Nour
