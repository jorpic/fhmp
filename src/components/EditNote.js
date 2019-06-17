// EditNote component provides textarea to enter a note and some logic to save
// it to storage.
// Drafts are saved to IDB after a small timeout.
// TODO:
// - drop draft on cancel

import cls from "classnames";
import {h, Component} from "preact";
import config from "../config";
import Page from "./Page";


export default class EditNote extends Component {
  constructor(props) {
    super(props);
    this.state = {
      text: "",
      draft: null, // used only to show message
      draftSaved: true
    };
  }


  componentDidMount() {
    const {noteId} = this.props;
    this.props.db.getDraft(noteId) // load draft if it exists
      .then(draft =>
        draft.text.length > 10 // draft is long enough to be useful
          ? this.setState({text: draft.text, draft, draftSaved: true})
          : this.loadSavedNote())
      .catch(this.loadSavedNote);

    // save draft every now and then
    this.draftSaveLoop = setInterval(
      () => this.state.draftSaved
        || this.props.db.saveDraft(noteId, this.state.text)
          .then(() => this.setState({draftSaved: true})),
      config.DRAFT_SAVE_TIMEOUT);
  }


  loadSavedNote = () =>
    this.props.noteId
      ? this.props.db.getNote(this.props.noteId)
          .then(note =>
            this.setState({draft: null, text: note ? note.text : ""}))
      : this.setState({draft: null, text: ""})

  dismissDraftMsg = () => this.setState({draft: null})


  componentWillUnmount() {
    clearInterval(this.draftSaveLoop);
    this.props.db.saveDraft(this.props.noteId, this.state.text)
      .catch(this.page.warning("Failed to save the draft"));
  }


  onText = ev => this.setState({
    text: ev.target.value,
    draftSaved: false,
  })


  onSave = () => {
    const {text} = this.state;
    const action = this.props.noteId
      ? () => this.props.db.updateNote(this.props.noteId, text)
      : () => this.props.db.createNote(text);
    action()
      .then(() => {
        this.props.db.dropDraft(this.props.noteId);
        this.setState({draftSaved: true, text: ""});
        this.page.success("Your note was saved successfully.")();
      })
      .catch(this.page.error(
          <span>
            Sorry! <br />
            We could not save your note to the local storage.
          </span>
      ));
  }


  render() {
    const textareaCls = cls(
      "textarea has-extra-height",
      {"is-success": this.state.draftSaved});

    return (
      <Page ref={ref => this.page = ref}>
        <div class="field">
          {this.state.draft && (
            <article class="message is-warning">
              <div>
                Found draft saved on {this.state.draft.time}
                <button class="button is-light" onClick={this.loadSavedNote}>
                  Drop it
                </button>
                <button class="button is-light" onClick={this.dismissDraftMsg}>
                  Keep it
                </button>
              </div>
            </article>
          )}
          <textarea class={textareaCls}
            ref={ref => this.textarea = ref}
            onInput={this.onText}
            value={this.state.text}
            autofocus="true"
          />
        </div>

        <nav class="navbar is-light is-fixed-bottom">
          <div class="navbar-brand">
            <a class="navbar-item is-expanded has-text-centered"
              disabled={!this.state.text}
              onClick={this.onSave}
            >
              Save
            </a>
          </div>
        </nav>
      </Page>);
  }
}
