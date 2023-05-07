export interface LoginDTO {
    id: number,
    token: string,
    username: string,
    role: string
}

export const saveLoginDTO = (loginDTO: LoginDTO) => {
    localStorage.setItem("id", loginDTO.id.toString());
    localStorage.setItem("token", loginDTO.token);
    localStorage.setItem("username", loginDTO.username);
    localStorage.setItem("role", loginDTO.role)
}

export const loadLoginDTO = (): LoginDTO | null => {
    let id = localStorage.getItem("id");
    let token = localStorage.getItem("token");
    let username = localStorage.getItem("username");
    let role = localStorage.getItem("role");

    if (!id || !token || !username || !role) {
        return null;
    }

    return {id: Number(id), token, username, role};
}

export const clearLoginDTO = () => {
    localStorage.removeItem("id");
    localStorage.removeItem("token");
    localStorage.removeItem("username");
    localStorage.removeItem("role");
}