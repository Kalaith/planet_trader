class AuthService {
  getToken(): string | null {
    try {
      const storedAuth = localStorage.getItem('auth-storage');
      if (!storedAuth) return null;
      const parsed = JSON.parse(storedAuth) as { state?: { token?: string | null } };
      return parsed.state?.token ?? null;
    } catch {
      return null;
    }
  }
  isAuthenticated(): boolean {
    return !!this.getToken();
  }
}
export default new AuthService();
