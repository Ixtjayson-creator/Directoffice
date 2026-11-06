import { useState } from 'react';
import { api, AuthResponse } from './api';

interface LoginScreenProps {
  onLogin: (auth: AuthResponse) => void;
}

export default function LoginScreen({ onLogin }: LoginScreenProps) {
  const [mode, setMode] = useState<'login' | 'setup'>('login');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  // Login fields
  const [officeCode, setOfficeCode] = useState('');
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');

  // Setup fields
  const [officeName, setOfficeName] = useState('');
  const [setupCode, setSetupCode] = useState('');
  const [adminUser, setAdminUser] = useState('');
  const [adminName, setAdminName] = useState('');
  const [adminPass, setAdminPass] = useState('');

  const inputStyle: React.CSSProperties = {
    width: '100%',
    padding: '10px 12px',
    marginBottom: '12px',
    border: '1px solid #d1d5db',
    borderRadius: '6px',
    fontSize: '14px',
    boxSizing: 'border-box',
  };

  const btnStyle: React.CSSProperties = {
    width: '100%',
    padding: '12px',
    backgroundColor: '#2563eb',
    color: '#fff',
    border: 'none',
    borderRadius: '6px',
    fontSize: '15px',
    fontWeight: 'bold',
    cursor: 'pointer',
  };

  async function handleLogin(e: React.FormEvent) {
    e.preventDefault();
    setError('');
    setLoading(true);
    try {
      const res = await api.login({ office_code: officeCode, username, password });
      onLogin(res);
    } catch (err: any) {
      setError(err.message || 'Login failed');
    } finally {
      setLoading(false);
    }
  }

  async function handleSetup(e: React.FormEvent) {
    e.preventDefault();
    setError('');
    setLoading(true);
    try {
      const res = await api.setupOffice({
        office_name: officeName,
        office_code: setupCode,
        admin_username: adminUser,
        admin_full_name: adminName,
        admin_password: adminPass,
      });
      onLogin(res);
    } catch (err: any) {
      setError(err.message || 'Setup failed');
    } finally {
      setLoading(false);
    }
  }

  return (
    <div style={{
      display: 'flex', alignItems: 'center', justifyContent: 'center',
      height: '100vh', width: '100vw', backgroundColor: '#f3f4f6'
    }}>
      <div style={{
        width: '400px', backgroundColor: '#fff', padding: '32px',
        borderRadius: '12px', boxShadow: '0 4px 24px rgba(0,0,0,0.1)'
      }}>
        <h1 style={{ textAlign: 'center', margin: '0 0 8px 0', fontSize: '24px' }}>DirectOffice</h1>
        <p style={{ textAlign: 'center', color: '#6b7280', margin: '0 0 24px 0' }}>
          {mode === 'login' ? 'Sign in to your office' : 'Create a new office'}
        </p>

        {error && (
          <div style={{
            padding: '10px', marginBottom: '16px', backgroundColor: '#fef2f2',
            color: '#dc2626', borderRadius: '6px', fontSize: '13px'
          }}>
            {error}
          </div>
        )}

        {mode === 'login' ? (
          <form onSubmit={handleLogin}>
            <input style={inputStyle} placeholder="Office Code" value={officeCode}
              onChange={e => setOfficeCode(e.target.value)} required />
            <input style={inputStyle} placeholder="Username" value={username}
              onChange={e => setUsername(e.target.value)} required />
            <input style={inputStyle} placeholder="Password" type="password" value={password}
              onChange={e => setPassword(e.target.value)} required />
            <button style={btnStyle} type="submit" disabled={loading}>
              {loading ? 'Signing in...' : 'Sign In'}
            </button>
          </form>
        ) : (
          <form onSubmit={handleSetup}>
            <input style={inputStyle} placeholder="Office Name" value={officeName}
              onChange={e => setOfficeName(e.target.value)} required />
            <input style={inputStyle} placeholder="Office Code" value={setupCode}
              onChange={e => setSetupCode(e.target.value)} required />
            <input style={inputStyle} placeholder="Admin Username" value={adminUser}
              onChange={e => setAdminUser(e.target.value)} required />
            <input style={inputStyle} placeholder="Admin Full Name" value={adminName}
              onChange={e => setAdminName(e.target.value)} required />
            <input style={inputStyle} placeholder="Admin Password" type="password" value={adminPass}
              onChange={e => setAdminPass(e.target.value)} required />
            <button style={btnStyle} type="submit" disabled={loading}>
              {loading ? 'Creating Office...' : 'Create Office'}
            </button>
          </form>
        )}

        <p style={{ textAlign: 'center', marginTop: '16px', fontSize: '13px' }}>
          {mode === 'login' ? (
            <>No office yet? <button onClick={() => { setMode('setup'); setError(''); }}
              style={{ background: 'none', border: 'none', color: '#2563eb', cursor: 'pointer', fontWeight: 'bold' }}>
              Create one</button></>
          ) : (
            <>Have an office? <button onClick={() => { setMode('login'); setError(''); }}
              style={{ background: 'none', border: 'none', color: '#2563eb', cursor: 'pointer', fontWeight: 'bold' }}>
              Sign in</button></>
          )}
        </p>
      </div>
    </div>
  );
}

