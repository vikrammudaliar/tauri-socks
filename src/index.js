const { invoke } = window.__TAURI__.core;
const container = document.getElementById('container');

document.getElementById('start').addEventListener('click', async () => {
    const statusDiv = document.getElementById('status');
    try {
        const response = await invoke('start_socks_proxy');
        statusDiv.textContent = 'Proxy is running';
        statusDiv.style.color = 'green';
        container.classList.remove('stopped');
        container.classList.add('started');
    } catch (error) {
        statusDiv.textContent = error;
        statusDiv.style.color = 'red';
    }
});

document.getElementById('stop').addEventListener('click', async () => {
    const statusDiv = document.getElementById('status');
    try {
        const response = await invoke('stop_socks_proxy');
        statusDiv.textContent = 'Proxy is not running';
        statusDiv.style.color = 'green';
        container.classList.remove('started');
        container.classList.add('stopped');
    } catch (error) {
        statusDiv.textContent = error;
        statusDiv.style.color = 'red';
    }
});
