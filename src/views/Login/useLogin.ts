import { invoke } from "@tauri-apps/api/core"; 

export const useLogin = () => {
  const login = async (user: string, password: string): Promise<boolean> => {
    console.log(user,password);
    return invoke<boolean>("login", {usuario: user, password: password})
  }

  return {
    login
  }
}