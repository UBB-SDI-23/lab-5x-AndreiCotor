import AccountComponent from "./AccountComponent";

export default function NavBar() {
    return (
        <nav className="navbar" role="navigation" aria-label="main-navigation">
            <div className="navbar-brand">
                <h1 className="tile">ABC</h1>
            </div>
            <div className="navbar-end mt-2 mb-2">
                <AccountComponent/>
            </div>
        </nav>
    );
}