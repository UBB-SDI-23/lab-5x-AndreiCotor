export default function NavMenu() {
    return (
        <aside className="menu">
            <p className="menu-label">General</p>
            <ul className="menu-list">
                <li><a href="/">Home</a></li>
                <li><a href="/problems">Problem Archive</a></li>
                <li><a href="/contests">Contests</a></li>
                <li><a href="/submissions">Evaluator</a></li>
            </ul>
        </aside>
    );
}