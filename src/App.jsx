import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import './App.css';

function App() {
  const [isMonitoring, setIsMonitoring] = useState(false);
  const [showNotification, setShowNotification] = useState(false);

  useEffect(() => {
    // Listen for screen time exceeded events
    const unlisten = listen('screen-time-exceeded', (event) => {
      setShowNotification(event.payload.exceeded);
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const toggleMonitoring = async () => {
    if (!isMonitoring) {
      await invoke('start_monitoring');
    } else {
      await invoke('stop_monitoring');
    }
    setIsMonitoring(!isMonitoring);
  };

  return (
    <div className="container">
      <div className={`notification-circle ${showNotification ? 'glow' : ''}`}>
        <button
          className="toggle-button"
          onClick={toggleMonitoring}
        >
          {isMonitoring ? 'Stop' : 'Start'}
        </button>
        {showNotification && (
          <div className="notification-message">
            Time to take a break and touch some grass! 🌿
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
