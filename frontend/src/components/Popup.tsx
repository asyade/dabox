export function Popup(props: {
    children: React.ReactNode;
}) {
    return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center">
            {props.children}
        </div>
    );
}
