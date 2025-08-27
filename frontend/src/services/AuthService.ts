class AuthService {
  private token: string | null = null;
  constructor() {
    this.token = localStorage.getItem('token');
  }
  getToken(): string | null {
    return this.token;
  }
  setToken(token: string) {
    this.token = token;
    localStorage.setItem('token', token);
  }
  clearToken() {
    this.token = null;
    localStorage.removeItem('token');
  }
  isAuthenticated(): boolean {
    return !!this.token;
  }
}
export default new AuthService();
