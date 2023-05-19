import {useCallback, useEffect, useState} from "react";
import useWebSocket, {ReadyState} from 'react-use-websocket';
import {socketUrl} from "../config";

export default function Chat() {
    const [message, setMessage] = useState<string>("");
    const [nickname, setNickname] = useState<string | undefined>();
    const [messageList, setMessageList] = useState<any>([]);
    const { sendMessage, lastMessage, readyState } = useWebSocket(socketUrl);

    useEffect(() => {
        if (lastMessage !== null) {
            setMessageList((prev:any) => prev.concat(JSON.parse(lastMessage.data)));
        }
    }, [lastMessage, setMessageList]);

    const handleSendMessage = useCallback(() => {
        let obj = {
            author: nickname,
            message: message
        };
        console.log(obj);
        let val = JSON.stringify(obj);
        console.log(val);
        sendMessage(val)
    }, [message, nickname, sendMessage]);

    const sendMessageFct = () => {
        const msg = {
            author: nickname,
            message: message,
            is_me: true
        };
        const newList = messageList.concat(msg);
        console.log(nickname);
        handleSendMessage();
        setMessageList(newList);
        setMessage("");
    }

    const messages = messageList.map((el: any) => {
        if (el.author === "Server") {
            return null;
        }
        return (
            <div className="mb-2">
                <p><b>{el.author + (el.is_me? " (You)": "") + ":"}</b> {el.message}</p>
            </div>
        );
    })

    return (
        <div>
            <h1 className="title">Chat</h1>
            <div className="columns">
                <div className="column is-half-desktop">
                    <div style={{minHeight: "500px", background: "beige", maxHeight: "500px", overflowY: "scroll"}}>
                        {messages}
                    </div>
                    <label className="label">{(nickname)?"Message": "Nickname"}</label>
                    <div className="columns">
                        <div className="control column is-10">
                            <input className="input"
                                   name="message"
                                   type="text"
                                   placeholder={(nickname)?"Message": "Nickname"}
                                   value={message}
                                   onChange={(e) => setMessage(e.target.value)}
                            />
                        </div>
                        <div className="column">
                            <button className="button is-link is-fullwidth" onClick={() => {
                                if (!nickname) {
                                    setNickname(message);
                                    setMessage("");
                                }
                                else {
                                    sendMessageFct();
                                }
                            }} disabled={readyState !== ReadyState.OPEN}>
                                {(nickname)?"Send": "Set name"}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}