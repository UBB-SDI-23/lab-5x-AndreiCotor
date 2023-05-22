import axios from "axios";

//export let axiosConfigured = axios.create({baseURL: "https://problemarena.mooo.com/api"});
export let axiosConfigured = axios.create({baseURL: "http://localhost:8000/api"});
//export let socketUrl = "wss://problemarena.mooo.com/api/chat/ws"
export let socketUrl = "ws://localhost:8000/api/chat/ws"