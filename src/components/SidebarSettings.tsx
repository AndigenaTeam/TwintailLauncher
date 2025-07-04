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
import {Settings} from "lucide-react";

export default function SidebarSettings({setOpenPopup, popup}: {setOpenPopup: (a: POPUPS) => void, popup: POPUPS}) {
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

    const {getReferenceProps, getFloatingProps} = useInteractions([
        hover
    ]);

    return (
        <React.Fragment>
            <Settings ref={refs.setReference} {...getReferenceProps()} className="text-white hover:text-white/55 w-8 h-10 mb-2 cursor-pointer flex-initial" onClick={() => {
                setOpenPopup(popup == POPUPS.NONE ? POPUPS.SETTINGS : POPUPS.NONE);
            }} />

            {(isOpen && popup == POPUPS.NONE) && (
                <div ref={refs.setFloating} style={floatingStyles} {...getFloatingProps()} className="bg-black/75 rounded-md p-2 w-full min-w-max z-50">
                    <FloatingArrow ref={arrowRef} context={context} className="fill-black/75" />
                    <span className="text-white z-50">Settings</span>
                </div>
            )}
        </React.Fragment>
    )
}
