import React from "react";
import {LoginDTO} from "../model/LoginDTO";

export const AuthContext = React.createContext({
    authContext: null as LoginDTO | null,
    setAuthContext: (() => {}) as any
});