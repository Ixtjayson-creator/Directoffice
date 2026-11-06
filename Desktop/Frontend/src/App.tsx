import { useState } from 'react';

export default function App() {
  const [activeTab, setActiveTab] = useState<'messages' | 'directory' | 'tasks'>('messages');

  return (
    <div style={{ display: 'flex', height: '100vh', width: '100vw', overflow: 'hidden' }}>
      {/* Sidebar navigation */}
      <nav style={{ width: '240px', backgroundColor: '#1f2937', color: '#fff', display: 'flex', flexDirection: 'column', padding: '16px' }}>
        <h2 style={{ fontSize: '18px', fontWeight: 'bold', margin: '0 0 24px 0' }}>DirectOffice</h2>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <button
            onClick={() => setActiveTab('messages')}
            style={{
              padding: '10px 14px',
              textAlign: 'left',
              backgroundColor: activeTab === 'messages' ? '#374151' : 'transparent',
              color: '#fff',
              border: 'none',
              borderRadius: '6px',
              cursor: 'pointer'
            }}
          >
            💬 Messages
          </button>
          <button
            onClick={() => setActiveTab('directory')}
            style={{
              padding: '10px 14px',
              textAlign: 'left',
              backgroundColor: activeTab === 'directory' ? '#374151' : 'transparent',
              color: '#fff',
              border: 'none',
              borderRadius: '6px',
              cursor: 'pointer'
            }}
          >
            👥 Directory
          </button>
          <button
            onClick={() => setActiveTab('tasks')}
            style={{
              padding: '10px 14px',
              textAlign: 'left',
              backgroundColor: activeTab === 'tasks' ? '#374151' : 'transparent',
              color: '#fff',
              border: 'none',
              borderRadius: '6px',
              cursor: 'pointer'
            }}
          >
            ✅ Tasks
          </button>
        </div>
      </nav>

      {/* Main content view */}
      <main style={{ flex: 1, padding: '24px', overflowY: 'auto' }}>
        {activeTab === 'messages' && (
          <div>
            <h1>Messages</h1>
            <p>DirectOffice desktop communication shell ready.</p>
          </div>
        )}
        {activeTab === 'directory' && (
          <div>
            <h1>Employee Directory</h1>
            <p>Office directory view.</p>
          </div>
        )}
        {activeTab === 'tasks' && (
          <div>
            <h1>Task Dashboard</h1>
            <p>Task and reminders view.</p>
          </div>
        )}
      </main>
    </div>
  );
}

