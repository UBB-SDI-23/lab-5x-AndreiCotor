import AccountComponent from "./AccountComponent";
import {useContext, useState} from "react";
import {AuthContext} from "../contexts/AuthContext";

export default function NavBar() {
    const [menuOpen, setMenuOpen] = useState<boolean>(false);
    const {authContext} = useContext(AuthContext);

    return (
        <nav className="navbar" role="navigation" aria-label="main-navigation">
            <div className="navbar-brand">
                <h1 className="tile">ABC</h1>
                <button className={"navbar-burger " + (menuOpen? "is-active": "")}
                        aria-label="menu"
                        aria-expanded="false"
                        data-target="navbarBasicExample"
                        onClick={() => setMenuOpen(!menuOpen)}
                >
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </button>
            </div>

            <div id="navbarBasicExample" className={"navbar-menu " + (menuOpen? "is-active": "")}>
                <div className="navbar-start is-hidden-desktop">
                    <a href="/" className="navbar-item">
                        Home
                    </a>
                    <a href="/problems" className="navbar-item">
                        All problems
                    </a>
                    <a href="/problems-by-success-rate" className="navbar-item">
                        Problems increasing by submissions
                    </a>
                    <a href="/contests" className="navbar-item">
                        Training Contests
                    </a>
                    <a href="/submissions" className="navbar-item">
                        Submissions
                    </a>
                    <a href="/users" className="navbar-item">
                        All users
                    </a>
                    <a href="/users-by-participation" className="navbar-item">
                        Users increasing by participations
                    </a>
                    <a href="/participations" className="navbar-item">
                        Participations
                    </a>
                    {(authContext && authContext.role === "admin") ?
                        (<a href="/admin" className="navbar-item">
                            Admin
                        </a>) : null
                    }
                </div>
                <div className="navbar-end">
                    <div className="navbar-item">
                        <div className="buttons">
                            <AccountComponent/>
                        </div>
                    </div>
                </div>
            </div>

        </nav>
    );
}