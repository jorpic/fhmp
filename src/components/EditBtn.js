import cls from "classnames";
import {h} from "preact";
import {route} from "preact-router";

export default function EditBtn(props) {
  const onEdit = () => route("/edit/" + props.noteId);
  return (
    <span
      class={cls("button edit-btn is-small", props.class)}
      onClick={onEdit}
    >
      <i class="fas fa-edit" />
    </span>);
}
