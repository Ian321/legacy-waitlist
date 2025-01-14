import { InfoNote } from "../../Components/NoteBox";
import { Highlight } from "../../Components/Form";
import { Copyable } from "../../Components/Copy";
import { ToastContext } from "../../contexts";
import React from "react";
import {
  CellHead,
  SmallCellHead,
  Table,
  TableHead,
  Row,
  TableBody,
  Cell,
} from "../../Components/Table";

export function ImplantTable({ type }) {
  const toastContext = React.useContext(ToastContext);
  var implants;
  if (type === "Amulet") {
    implants = [
      "High-grade Amulet Alpha",
      "High-grade Amulet Beta",
      "High-grade Amulet Gamma",
      "High-grade Amulet Delta",
      "High-grade Amulet Epsilon",
    ];
  } else {
    implants = [
      "High-grade Ascendancy Alpha",
      "High-grade Ascendancy Beta",
      "High-grade Ascendancy Gamma",
      "High-grade Ascendancy Delta",
      "High-grade Ascendancy Epsilon",
    ];
  }
  return (
    <>
      <InfoNote>
        {type === "Amulet"
          ? "Amulet tagged fits require at least Amulet 1 - 5 to be flown."
          : "Required for Elite badge on non implant specific ships."}
      </InfoNote>

      <Table style={{ width: "100%" }}>
        <TableHead>
          <Row>
            <SmallCellHead></SmallCellHead>
            <CellHead>DEFAULT</CellHead>
            <CellHead>ALTERNATIVE (not required)</CellHead>
          </Row>
        </TableHead>
        <TableBody>
          {implants.map((currentValue, index) => (
            <ImplantAllRow
              key={index}
              toast={toastContext}
              slot={index + 1}
              implant={currentValue}
            />
          ))}

          <Row>
            <Cell>
              <b>Slot 6</b>
            </Cell>
            <Cell>
              <CopyImplantText toast={toastContext} item={"WS-618"} /> increased warp speed.
            </Cell>
            {type === "Amulet" ? (
              <Cell></Cell>
            ) : (
              <Cell>
                <CopyImplantText toast={toastContext} item={"High-grade Ascendancy Omega"} /> if you
                have too much isk, increased warp speed.
              </Cell>
            )}
          </Row>

          <HardWires toastContext={toastContext} />
        </TableBody>
      </Table>
    </>
  );
}

function ImplantAllRow({ toast, slot, implant }) {
  return (
    <Row>
      <Cell>
        <b>Slot {slot}</b>
      </Cell>
      <Cell>
        <CopyImplantText toast={toast} item={implant} />
      </Cell>

      <Cell></Cell>
    </Row>
  );
}

function CopyImplantText({ toast, item }) {
  return (
    <Highlight
      onClick={(evt) => {
        Copyable(toast, item);
      }}
    >
      {item}
    </Highlight>
  );
}

function HardWires({ toastContext }) {
  return (
    <>
      <Row>
        <Cell>
          <b>Slot 7</b>
        </Cell>
        <Cell>
          <CopyImplantText toast={toastContext} item={"Ogdin's Eye"} /> increased tracking.
        </Cell>

        <Cell>
          <CopyImplantText toast={toastContext} item={"MR-706"} /> equal to Ogdin&apos;s. <br />
          <CopyImplantText toast={toastContext} item={"RA-706"} /> reps will use less cap, for
          <b> logi only pilots.</b>
        </Cell>
      </Row>
      <Row>
        <Cell>
          <b>Slot 8</b>
        </Cell>
        <Cell>
          <CopyImplantText toast={toastContext} item={"EM-806"} /> increased capacitor.
        </Cell>

        <Cell>
          <CopyImplantText toast={toastContext} item={"MR-807"} /> longer webbing range, for
          <b> vindicator only pilots.</b> <br />
          <CopyImplantText toast={toastContext} item={"Zor's Custom Navigation Hyper-Link"} />{" "}
          increased MWD speed for
          <b> DPS only pilots.</b>
        </Cell>
      </Row>
      <Row>
        <Cell>
          <b>Slot 9</b>
        </Cell>
        <Cell>
          <CopyImplantText toast={toastContext} item={"RF-906"} /> increased rate of fire.
        </Cell>

        <Cell>
          <CopyImplantText toast={toastContext} item={"Pashan's Turret Customization Mindlink"} />{" "}
          if you have too much isk, increased rate of fire.
        </Cell>
      </Row>
      <Row>
        <Cell>
          <b>Slot 10</b>
        </Cell>
        <Cell>
          <b>Kronos/Vindicator:</b>
          <br />
          <CopyImplantText toast={toastContext} item={"LH-1006"} /> increased hybrid weapon damage.{" "}
          <br />
          <br />
          <b>Paladin/Nightmare:</b>
          <br />
          <CopyImplantText toast={toastContext} item={"LE-1006"} /> increased energy weapon damage.
        </Cell>
        <Cell>
          <CopyImplantText toast={toastContext} item={"HG-1006"} /> or
          <br />
          <CopyImplantText toast={toastContext} item={"HG-1008"} /> if you have too much isk,
          increased RAW armor HP for
          <b> logi only pilots.</b> <br />
          <br />
          <b>Paladin/Nightmare:</b>
          <br />
          <CopyImplantText toast={toastContext} item={"Pashan's Turret Handling Mindlink"} /> if you
          have too much isk, increased weapon damage.
        </Cell>
      </Row>
    </>
  );
}
