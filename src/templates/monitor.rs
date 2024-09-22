/// Get the HTML content to render the monitor page.
///
/// # See Also
///
/// - This page is served as a response for the `/` entry point once authenticated.
///
/// # Returns
///
/// A `String` version of the HTML, CSS and JS content.
pub fn get_content() -> String {
    r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
    <title>SysMonk - System Monitor - v{{ version }}</title>
    <meta property="og:type" content="SystemMonitor">
    <meta name="keywords" content="Rust, Monitor, actix, JavaScript, HTML, CSS">
    <meta name="author" content="Vignesh Rao">
    <!-- Favicon.ico and Apple Touch Icon -->
    <link rel="icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.ico">
    <link rel="apple-touch-icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.png">
    <meta content="width=device-width, initial-scale=1" name="viewport">
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <!-- CSS and JS for night mode -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/jquery/2.2.2/jquery.min.js"></script>
    <script type="text/javascript" src="https://thevickypedia.github.io/open-source/nightmode/night.js" defer></script>
    <link rel="stylesheet" type="text/css" href="https://thevickypedia.github.io/open-source/nightmode/night.css">
    <!-- Font Awesome icons -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.6.0/css/font-awesome.min.css">
    <!--suppress CssUnusedSymbol -->
    <style id="main-css">
        body {
            font-family: Arial, sans-serif;
            overflow-x: hidden;
        }

        .docker-stats {
            height: 100%;
            margin: 0;
            display: none;  /* Hide the container initially */
            align-items: center;
            justify-content: center;
            flex-direction: column;  /* Ensure vertical alignment */
        }

        .docker-stats h3 {
            text-align: center;
            margin-bottom: 20px;
        }

        table {
            width: 80%;
            border-collapse: collapse;
            display: none;  /* Hide the table initially */
        }

        table, th, td {
            border: 1px solid #ccc;
        }

        th, td {
            padding: 10px;
            text-align: left;
        }

        .container {
            display: flex;
            justify-content: space-between;
            margin-top: 50px;
        }

        .box {
            border: 1px solid #ccc;
            padding: 20px;
            width: 30%;
            text-align: center;
            margin: 1%;
        }

        .progress {
            width: 100%;
            background-color: transparent;
            border-radius: 5px;
            overflow: hidden;
            transition: background-color 0.5s ease;
        }

        .progress-bar {
            height: 25px;
            transition: width 0.5s ease, background-color 0.5s ease;
            width: 0;
        }

        .progress-bar-green {
            background-color: #4caf50;
        }

        .progress-bar-yellow {
            background-color: #ffeb3b;
        }

        .progress-bar-orange {
            background-color: #ff9800;
        }

        .progress-bar-red {
            background-color: #f44336;
        }

        .chart-container {
            position: relative;
            height: 200px;
            width: 80%;
            margin: 0 auto;
            max-width: 100%;
        }

        canvas {
            width: 100% !important;
            height: inherit !important;
            max-height: 100% !important;
        }

        .tooltip-button {
            padding: 5px 5px;
            font-size: 14px;
            cursor: pointer;
            border: 1px solid #ccc;
            border-radius: 5px;
            background-color: #f0f0f0;
        }

        .tooltip-button:hover {
            background-color: #e0e0e0;
        }

        .center-container {
            width: 100%;
            margin-left: 45%;
        }

        .center-container details {
            text-align: left;
        }

        h1 {
            width: 100%;
            text-align: center;
            align-content: center;
        }

        .docs {
            position: absolute;
            top: 3.8%;
            right: 230px;
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
        }

        .redoc {
            position: absolute;
            top: 3.8%;
            right: 130px;
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
        }

        .logout {
            position: absolute;
            top: 3.8%;
            right: 30px;
            border: none;
            padding: 10px 14px;
            font-size: 16px;
            cursor: pointer;
        }

        .corner {
            position: absolute;
            top: 8%;
            right: 2%;
            font-size: 12px;
        }

        .graph-canvas {
            max-width: 600px;
        }

    </style>
</head>
<div class="corner">
    <i>
        <span id="span"></span><br>
        Last status update: <span id="lastUpdate"></span><br>
        Next update: <span id="nextUpdate"></span><br>
        Generated by <a href="https://github.com/thevickypedia/SysMonk/releases/tag/v{{ version }}">SysMonk - v{{
        version }}</a>
    </i>
</div>
<body translate="no">
<div class="toggler fa fa-moon-o"></div>
<button class="docs" onclick="goDocs()"><i class="fa fa-book"></i> Docs</button>
<button class="redoc" onclick="goReDoc()"><i class="fa fa-file"></i> ReDoc</button>
<button class="logout" onclick="logOut()"><i class="fa fa-sign-out"></i> Logout</button>
<h1>SysMonk - System Monitor</h1>
<div class="center-container">
    <details>
        <summary><strong>System Information (Basic)</strong></summary>
        <br>
        {% for key, value in sys_info_basic|items() %}
        <strong>{{ key|capwords }}: </strong>{{ value }}<br>
        {% endfor %}
    </details>
    <br>
    <details>
        <summary><strong>System Information (Memory & Storage)</strong></summary>
        <br>
        {% for key, value in sys_info_mem_storage|items() %}
        <strong>{{ key|capwords }}: </strong>{{ value }}<br>
        {% endfor %}
    </details>
    <br>
    <details>
        <summary><strong>System Information (Network)</strong></summary>
        <br>
        {% for key, value in sys_info_network|items() %}
        <strong>{{ key|capwords }}: </strong>{{ value }}<br>
        {% endfor %}
    </details>
    {% if sys_info_disks %}
    <br>
    <details>
        <summary><strong>System Information (Disks)</strong></summary>
        {% for disk_info in sys_info_disks %}
        <br>
        {% for key, value in disk_info|items() %}
        <strong>{{ key }}: </strong>{{ value }}<br>
        {% endfor %}
        {% endfor %}
    </details>
    {% endif %}
</div>
<div class="container">
    <!-- Box to display utilization per CPU -->
    <div class="box">
        <h3>CPU Usage</h3>
        <div class="cpu-box" id="cpuUsageContainer">
            <!-- CPU Usage will be dynamically added here -->
        </div>
    </div>
    <!-- Box to display Memory and Swap usage along with CPU load avg -->
    <div class="box">
        <h3>Memory Usage</h3>
        <div class="progress">
            <div id="memoryUsage" class="progress-bar"></div>
        </div>
        <p id="memoryUsageText">Memory: 0%</p>

        {% if 'swap' in sys_info_mem_storage %}
        <h3>Swap Usage</h3>
        <div class="progress">
            <div id="swapUsage" class="progress-bar"></div>
        </div>
        <p id="swapUsageText">Swap: 0%</p>
        {% endif %}

        <div class="graph">
            <h3>CPU Load Averages</h3>
            <canvas class="graph-canvas" id="loadChart" width="400" height="200"></canvas>
        </div>
    </div>
    <!-- Box to display Memory and Swap usage as Pie charts -->
    <div class="box">
        <h3>Memory Usage</h3>
        <h5 id="memoryTotal"></h5>
        <div class="chart-container">
            <canvas id="memoryChart"></canvas>
        </div>
        {% if 'swap' in sys_info_mem_storage %}
        <h3>Swap Usage</h3>
        <h5 id="swapTotal"></h5>
        <div class="chart-container">
            <canvas id="swapChart"></canvas>
        </div>
        {% endif %}
    </div>
</div>
<div id="docker-stats" class="docker-stats">
    <h3>Docker Stats</h3>
    <table id="dockerStatsTable">
        <thead>
            <tr>
                <th>Container ID</th>
                <th>Container Name</th>
                <th>CPU %</th>
                <th>Memory Usage</th>
                <th>Memory %</th>
                <th>Net I/O</th>
                <th>Block I/O</th>
                <th>PIDs</th>
            </tr>
        </thead>
        <tbody>
        </tbody>
    </table>
</div>
<script>
    document.addEventListener('DOMContentLoaded', function () {
        const wsProtocol = window.location.protocol === "https:" ? "wss" : "ws";
        const wsHost = window.location.host;
        const ws = new WebSocket(`${wsProtocol}://${wsHost}/ws/system`);

        ws.onopen = () => {
            console.log('WebSocket connection established');
        };
        ws.onclose = () => {
            console.log('WebSocket connection closed');
            alert('WebSocket connection closed. Please login again.');
            logOut();
            return;
        };

        let memoryChartInstance = null;
        let swapChartInstance = null;
        let loadChartInstance = null;

        ws.onmessage = function (event) {
            let data;
            try {
                data = JSON.parse(event.data);
            } catch (error) {
                console.warn('Error parsing JSON data:', error);
                alert(event.data);
                logOut();
                return;
            }

            const dockerStatsJSON = data.docker_stats;
            // Check if dockerStatsJSON is valid
            if (dockerStatsJSON && dockerStatsJSON.length > 0) {
                // Show the container and the table
                const statsContainer = document.getElementById("docker-stats");
                statsContainer.style.display = "flex";
                const table = document.getElementById("dockerStatsTable");
                table.style.display = "table";
                // Get reference to the table body
                const tableBody = document.querySelector('#dockerStatsTable tbody');
                // Clear the existing table rows
                tableBody.innerHTML = '';
                // Loop through the JSON data and populate the table
                dockerStatsJSON.forEach(container => {
                    const row = document.createElement('tr');
                    row.innerHTML = `
                        <td>${container.ID}</td>
                        <td>${container.Name}</td>
                        <td>${container.CPUPerc}</td>
                        <td>${container.MemUsage}</td>
                        <td>${container.MemPerc}</td>
                        <td>${container.NetIO}</td>
                        <td>${container.BlockIO}</td>
                        <td>${container.PIDs}</td>
                    `;
                    tableBody.appendChild(row);
                });
            } else {
                // Hide the container if no data is available
                document.getElementById("docker-stats").style.display = "none";
            }

            // Floating info on top right of the page
            const date = new Date();
            document.getElementById('lastUpdate').innerText = date.toLocaleString();
            // Convert seconds to milliseconds
            const refreshInterval = data.refresh_interval * 1000;
            const nextUpdateTime = new Date(date.getTime() + refreshInterval);
            document.getElementById('nextUpdate').innerText = nextUpdateTime.toLocaleString();

            // Update CPU usage
            const cpuUsage = data.cpu_usage;
            const cpuContainer = document.getElementById('cpuUsageContainer');
            cpuContainer.innerHTML = ''; // Clear previous content
            cpuUsage.forEach((usage, index) => {
                const cpuDiv = document.createElement('div');
                cpuDiv.innerHTML = `
                        <strong>CPU ${index + 1}:</strong> ${usage}%
                        <div class="progress">
                            <div id="cpu${index}" class="progress-bar"></div>
                        </div>
                    `;
                cpuContainer.appendChild(cpuDiv);
                updateProgressBar(`cpu${index}`, usage);
            });

            // Memory Usage Progress Bar
            const memoryInfo = data.memory_info;
            const memoryUsage = (memoryInfo.used / memoryInfo.total) * 100;
            document.getElementById('memoryUsage').style.width = memoryUsage.toFixed(2) + '%';
            document.getElementById('memoryUsageText').innerText = `Memory: ${memoryUsage.toFixed(2)}%`;
            updateProgressBar('memoryUsage', memoryUsage);

            // Swap Usage Progress Bar
            const swapInfo = data.swap_info;
            if (swapInfo) {
                const swapUsage = (swapInfo.used / swapInfo.total) * 100;
                document.getElementById('swapUsage').style.width = swapUsage.toFixed(2) + '%';
                document.getElementById('swapUsageText').innerText = `Swap: ${swapUsage.toFixed(2)}%`;
                updateProgressBar('swapUsage', swapUsage);
            }

            // CPU Load Avg Graph
            const loadAverages = data.load_averages;
            if (loadChartInstance) {
                loadChartInstance.data.datasets[0].data = [loadAverages["m1"], loadAverages["m5"], loadAverages["m15"]];
                loadChartInstance.update();
            } else {
                const ctx = document.getElementById('loadChart').getContext('2d');
                loadChartInstance = new Chart(ctx, {
                    type: 'bar',
                    data: {
                        labels: ['1 minute', '5 minutes', '15 minutes'],
                        datasets: [{
                            label: 'Load Average',
                            data: [loadAverages["m1"], loadAverages["m5"], loadAverages["m15"]],
                            backgroundColor: [
                                'rgba(75, 192, 192, 0.2)',
                                'rgba(153, 102, 255, 0.2)',
                                'rgba(255, 159, 64, 0.2)'
                            ],
                            borderColor: [
                                'rgba(75, 192, 192, 1)',
                                'rgba(153, 102, 255, 1)',
                                'rgba(255, 159, 64, 1)'
                            ],
                            borderWidth: 1
                        }]
                    },
                    options: {
                        plugins: {
                            // Hide the legend
                            legend: {
                                display: false
                            }
                        },
                        scales: {
                            y: {
                                beginAtZero: true,
                                title: {
                                    display: true,
                                    text: 'Number of Processes'
                                },
                                ticks: {
                                    // Set integer step size
                                    stepSize: 1,
                                    callback: function (value) {
                                        return Number.isInteger(value) ? value : '';
                                    }
                                }
                            }
                        }
                    }
                });
            }

            // Memory Chart
            document.getElementById("memoryTotal").innerText = `Total: ${formatBytes(memoryInfo.total)}`;
            if (memoryChartInstance) {
                memoryChartInstance.data.datasets[0].data = [memoryInfo.used, memoryInfo.total - memoryInfo.used];
                memoryChartInstance.update();
            } else {
                const memoryChart = document.getElementById('memoryChart').getContext('2d');
                memoryChartInstance = new Chart(memoryChart, {
                    type: 'pie',
                    data: {
                        labels: ['Used', 'Free'],
                        datasets: [{
                            label: 'Memory Usage',
                            data: [memoryInfo.used, memoryInfo.total - memoryInfo.used],
                            backgroundColor: ['#FF6384', '#36A2EB']
                        }]
                    },
                    options: {
                        responsive: true,
                        plugins: {
                            tooltip: {
                                callbacks: {
                                    label: function (tooltipItem) {
                                        const value = tooltipItem.raw;
                                        const formattedValue = formatBytes(value);
                                        return `${tooltipItem.label}: ${formattedValue}`;
                                    }
                                }
                            }
                        }
                    }
                });
            }

            // Swap Chart
            const swapChart = document.getElementById('swapChart');
            if (swapChart) {
                document.getElementById("swapTotal").innerText = `Total: ${formatBytes(swapInfo.total)}`;
            }
            if (swapChartInstance) {
                swapChartInstance.data.datasets[0].data = [swapInfo.used, swapInfo.total - swapInfo.used];
                swapChartInstance.update();
            } else {
                if (swapChart) {
                    const swapContext = swapChart.getContext('2d')
                    swapChartInstance = new Chart(swapContext, {
                        type: 'pie',
                        data: {
                            labels: ['Used', 'Free'],
                            datasets: [{
                                label: 'Swap Usage',
                                data: [swapInfo.used, swapInfo.total - swapInfo.used],
                                backgroundColor: ['#FFCE56', '#E7E9ED']
                            }]
                        },
                        options: {
                            responsive: true,
                            plugins: {
                                tooltip: {
                                    callbacks: {
                                        label: function (tooltipItem) {
                                            const value = tooltipItem.raw;
                                            const formattedValue = formatBytes(value);
                                            return `${tooltipItem.label}: ${formattedValue}`;
                                        }
                                    }
                                }
                            }
                        }
                    });
                }
            }

        };

        function updateProgressBar(id, percentage) {
            const bar = document.getElementById(id);
            bar.style.width = percentage + '%';

            // Remove old color classes
            bar.classList.remove('progress-bar-green', 'progress-bar-yellow', 'progress-bar-orange', 'progress-bar-red');

            // Add new color class based on percentage
            if (percentage <= 50) {
                bar.classList.add('progress-bar-green');
            } else if (percentage <= 70) {
                bar.classList.add('progress-bar-yellow');
            } else if (percentage <= 90) {
                bar.classList.add('progress-bar-orange');
            } else {
                bar.classList.add('progress-bar-red');
            }
        }

        function formatBytes(bytes) {
            if (bytes < 1024) return bytes + ' bytes';
            else if (bytes < 1048576) return (bytes / 1024).toFixed(2) + ' KB';
            else if (bytes < 1073741824) return (bytes / 1048576).toFixed(2) + ' MB';
            else return (bytes / 1073741824).toFixed(2) + ' GB';
        }
    });

    function logOut() {
        window.location.href = window.location.origin + "{{ logout }}";
    }

    function goDocs() {
        window.location.href = window.location.origin + "/docs";
    }

    function goReDoc() {
        window.location.href = window.location.origin + "/redoc";
    }
</script>
</body>
</html>
"###.to_string()
}
