import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

interface Todo {
  id: number;
  title: string;
  completed: boolean;
}

function App() {
  const [connectionId, setConnectionId] = useState<string | null>(null);
  const [todos, setTodos] = useState<Todo[]>([]);
  const [newTodo, setNewTodo] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (connectionId) {
      loadTodos();
    }
  }, [connectionId]);

  async function initializeDb() {
    try {
      setLoading(true);
      const connId = await invoke<string>('create_demo_db');
      setConnectionId(connId);
      console.log('Database initialized with connection ID:', connId);
    } catch (error) {
      console.error('Failed to initialize database:', error);
    } finally {
      setLoading(false);
    }
  }

  async function loadTodos() {
    if (!connectionId) return;

    try {
      setLoading(true);
      const result = await invoke<{ columns: string[]; rows: any[][] }>('get_todos', {
        connectionId,
      });

      // 将结果转换为 Todo 对象数组
      const loadedTodos: Todo[] = result.rows.map((row) => ({
        id: row[0],
        title: row[1],
        completed: Boolean(row[2]),
      }));

      setTodos(loadedTodos);
    } catch (error) {
      console.error('Failed to load todos:', error);
    } finally {
      setLoading(false);
    }
  }

  async function addTodo() {
    if (!connectionId || !newTodo.trim()) return;

    try {
      setLoading(true);
      await invoke('add_todo', {
        connectionId,
        title: newTodo,
      });
      setNewTodo('');
      await loadTodos();
    } catch (error) {
      console.error('Failed to add todo:', error);
    } finally {
      setLoading(false);
    }
  }

  async function toggleTodo(id: number, completed: boolean) {
    if (!connectionId) return;

    try {
      setLoading(true);
      await invoke('toggle_todo', {
        connectionId,
        id,
        completed: !completed,
      });
      await loadTodos();
    } catch (error) {
      console.error('Failed to toggle todo:', error);
    } finally {
      setLoading(false);
    }
  }

  async function closeDatabase() {
    if (!connectionId) return;

    try {
      setLoading(true);
      await invoke('close_db', { connectionId });
      setConnectionId(null);
      setTodos([]);
    } catch (error) {
      console.error('Failed to close database:', error);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="container">
      <h1>LibSQL Todo Example</h1>

      {connectionId ? (
        <div className="todo-app">
          <div className="add-todo">
            <input
              type="text"
              placeholder="新增待办事项..."
              value={newTodo}
              onChange={(e) => setNewTodo(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && addTodo()}
            />
            <button onClick={addTodo} disabled={loading}>
              添加
            </button>
          </div>

          {loading && <p className="loading">Loading...</p>}

          <ul className="todo-list">
            {todos.map((todo) => (
              <li key={todo.id} className={todo.completed ? 'completed' : ''}>
                <label>
                  <input
                    type="checkbox"
                    checked={todo.completed}
                    onChange={() => toggleTodo(todo.id, todo.completed)}
                    disabled={loading}
                  />
                  <span>{todo.title}</span>
                </label>
              </li>
            ))}
          </ul>

          {todos.length === 0 && !loading && <p>暂无待办事项</p>}

          <div className="actions">
            <button onClick={loadTodos} disabled={loading}>
              刷新
            </button>
            <button onClick={closeDatabase} disabled={loading} className="close-db">
              关闭数据库
            </button>
          </div>
        </div>
      ) : (
        <div className="init-db">
          <p>点击下方按钮初始化示例数据库</p>
          <button onClick={initializeDb} disabled={loading}>
            {loading ? 'Loading...' : '初始化数据库'}
          </button>
        </div>
      )}

      <p className="status">
        状态: {connectionId ? `已连接 (ID: ${connectionId})` : '未连接'}
      </p>
    </div>
  );
}

export default App; 