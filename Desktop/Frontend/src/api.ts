const API_BASE_URL = 'http://localhost:8080/api';

export interface OfficeSetupRequest {
  office_name: string;
  office_code: string;
  timezone?: string;
  admin_username: string;
  admin_full_name: string;
  admin_password: string;
}

export interface LoginRequest {
  office_code: string;
  username: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: {
    id: string;
    organization_id: string;
    department_id?: string | null;
    username: string;
    full_name: string;
    role: string;
    status: string;
  };
  organization: {
    id: string;
    name: string;
    office_code: string;
    timezone: string;
  };
}

export interface Conversation {
  id: string;
  is_group: boolean;
  name?: string | null;
}

export interface Message {
  id: string;
  conversation_id: string;
  sender_id: string;
  content: string;
  timestamp: string;
  status: string;
}

export interface TaskItem {
  id: string;
  title: string;
  description?: string | null;
  status: string;
  created_by: string;
  assigned_to?: string | null;
  due_date?: string | null;
  created_at: string;
}

class ApiClient {
  private token: string | null = null;

  setToken(token: string | null) {
    this.token = token;
    if (token) {
      localStorage.setItem('directoffice_token', token);
    } else {
      localStorage.removeItem('directoffice_token');
    }
  }

  getToken(): string | null {
    if (!this.token) {
      this.token = localStorage.getItem('directoffice_token');
    }
    return this.token;
  }

  private async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...((options.headers as Record<string, string>) || {}),
    };

    const token = this.getToken();
    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }

    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      ...options,
      headers,
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(errorText || `HTTP Error ${response.status}`);
    }

    return response.json();
  }

  async healthCheck(): Promise<{ status: string; service: string }> {
    return this.request('/health', { method: 'GET' });
  }

  async login(payload: LoginRequest): Promise<AuthResponse> {
    const res = await this.request<AuthResponse>('/auth/login', {
      method: 'POST',
      body: JSON.stringify(payload),
    });
    this.setToken(res.token);
    return res;
  }

  async setupOffice(payload: OfficeSetupRequest): Promise<AuthResponse> {
    const res = await this.request<AuthResponse>('/office/setup', {
      method: 'POST',
      body: JSON.stringify(payload),
    });
    this.setToken(res.token);
    return res;
  }

  async createConversation(is_group: boolean, name?: string): Promise<Conversation> {
    return this.request<Conversation>('/messaging/conversation/create', {
      method: 'POST',
      body: JSON.stringify({ is_group, name }),
    });
  }

  async sendMessage(conversation_id: string, content: string): Promise<Message> {
    return this.request<Message>('/messaging/message/send', {
      method: 'POST',
      body: JSON.stringify({ conversation_id, content }),
    });
  }

  async getMessages(conversation_id: string): Promise<Message[]> {
    return this.request<Message[]>(`/messaging/message/${conversation_id}`, {
      method: 'GET',
    });
  }

  async createTask(task: {
    title: string;
    description?: string;
    assigned_to?: string;
    due_date?: string;
  }): Promise<TaskItem> {
    return this.request<TaskItem>('/task/create', {
      method: 'POST',
      body: JSON.stringify(task),
    });
  }

  async getTask(task_id: string): Promise<TaskItem> {
    return this.request<TaskItem>(`/task/${task_id}`, { method: 'GET' });
  }

  async updateTask(
    task_id: string,
    updates: { title?: string; description?: string; status?: string; due_date?: string }
  ): Promise<TaskItem> {
    return this.request<TaskItem>(`/task/${task_id}`, {
      method: 'PUT',
      body: JSON.stringify(updates),
    });
  }

  async generateInvite(payload: {
    department_id?: string;
    role: string;
    expires_in_hours?: number;
  }): Promise<{ invite_code: string; expires_at: string }> {
    return this.request('/invitation/generate', {
      method: 'POST',
      body: JSON.stringify(payload),
    });
  }

  async transcribeAudio(audio_path: string): Promise<{ transcript: string }> {
    return this.request('/ai/transcribe', {
      method: 'POST',
      body: JSON.stringify({ audio_path }),
    });
  }

  async summarizeText(text: string): Promise<{ summary: string }> {
    return this.request('/ai/summarize', {
      method: 'POST',
      body: JSON.stringify({ text }),
    });
  }
}

export const api = new ApiClient();

