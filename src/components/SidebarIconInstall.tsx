import React, {useRef, useState} from "react";
import {
    arrow,
    autoUpdate,
    flip,
    FloatingArrow,
    offset,
    shift,
    useFloating,
    useHover,
    useInteractions
} from "@floating-ui/react";
import {POPUPS} from "./popups/POPUPS.ts";

type SidebarIconProps = {
    icon: string,
    name: string,
    id: string,
    background: string,
    enabled: boolean,
    setCurrentInstall: (a: string) => void,
    setOpenPopup: (a: POPUPS) => void,
    popup: POPUPS,
    setDisplayName: (name: string) => void,
    setBackground: (file: string) => void,
    setGameIcon: (a: string) => void,
}

export default function SidebarIconInstall({icon, name, id, setCurrentInstall, setGameIcon, setOpenPopup, popup, setDisplayName, setBackground, background, enabled}: SidebarIconProps) {
    const [isOpen, setIsOpen] = useState(false);

    const arrowRef = useRef(null);
    const {refs, floatingStyles, context} = useFloating({
        open: isOpen,
        onOpenChange: setIsOpen,
        middleware: [offset(25), flip(), shift(), arrow({
            element: arrowRef
        })],
        whileElementsMounted: autoUpdate,
        placement: "right",
    });

    const hover = useHover(context, {move: false});
    const {getReferenceProps, getFloatingProps} = useInteractions([hover]);

    return (
        <React.Fragment>
            {(enabled) ? <img ref={refs.setReference} {...getReferenceProps()} id={`${id}`} className={`aspect-square w-12 rounded-lg cursor-pointer hover:border-blue-600 hover:border-2 focus:border-2 focus:border-blue-600 outline-none disabled:cursor-not-allowed disabled:border-0`} srcSet={undefined} loading={"lazy"} decoding={"async"} src={icon} tabIndex={0} onClick={() => {
                let elem = document.getElementById(id);
                // @ts-ignore
                if (elem.hasAttribute("disabled")) {} else {
                    setOpenPopup(POPUPS.NONE)
                    setCurrentInstall(id)
                    setDisplayName(name)
                    setBackground(background)
                    setGameIcon(icon)
                    // @ts-ignore
                    elem.focus();
                }
            }} alt={"?"}/> : null}
            {(enabled) ?
                (isOpen && popup == POPUPS.NONE) && (
                    <div ref={refs.setFloating} style={floatingStyles} {...getFloatingProps()} className="bg-black/75 rounded-md p-2 w-full min-w-max z-50">
                        <FloatingArrow ref={arrowRef} context={context} className="fill-black/75" />
                        <span className="text-white z-50">{name}</span>
                    </div>
                ) : null}
        </React.Fragment>
    )
}
