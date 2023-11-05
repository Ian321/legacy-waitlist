import { useContext, useState } from "react"
import { apiCall, errorToaster } from "../../../../../api";
import { ToastContext } from "../../../../../contexts"
import { Button } from "./Button";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faSpinner, faPaperPlane } from "@fortawesome/free-solid-svg-icons"

async function invite(id, character_id) {
  return await apiCall("/api/waitlist/invite", {
    json: { id, character_id },
  });
}

const InviteButton = ({ fitId, isRejected, bossId }) => {
  const [ pending, isPending ] = useState(false);
  const toastContext = useContext(ToastContext);

  const handleClick = () => {
    isPending(true);
    errorToaster(
      toastContext,
      invite(fitId, bossId)
      .finally(_ => isPending(false))
    );
  }


  return (
    <>
      <Button type="button"
        variant="primary"
        data-tooltip-id="tip"
        data-tooltip-html={!isRejected ? "Invite" : "Fit rejected"}
        disabled={isRejected || pending}
        onClick={handleClick}
      >
         <FontAwesomeIcon fixedWidth icon={!pending ? faPaperPlane : faSpinner} spin={pending} />
      </Button>
    </>
  )
}

export default InviteButton;