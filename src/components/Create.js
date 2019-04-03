import cls from "classnames";
import { h, Component } from "preact";


export default class Create extends Component {
  constructor(props) {
    super(props);
    this.state = {
      error: null,
      text: "",
      draftSaved: true
    };
  }

  componentDidMount() {
    // load draft from IDB
    this.props.db.getDraft()
      .then(draft => draft !== undefined
        && this.setState({text: draft.text}));

    // save draft every now and then
    // FIXME: handle "can't save" errors?
    const DRAFT_SAVE_TIMEOUT = 4; // seconds
    this.draftSaveLoop = setInterval(
      () => this.state.draftSaved
        || this.props.db.saveDraft(this.state.text)
          .then(() => this.setState({draftSaved: true})),
      DRAFT_SAVE_TIMEOUT*1000);
  }

  componentWillUnmount() {
    clearInterval(this.draftSaveLoop);
    this.props.db.saveDraft(this.state.text);
  }


  // called by parent to set focus in textarea
  focus() {
    this.textarea.focus();
  }


  onText = ev =>
    this.setState({
      text: ev.target.value,
      draftSaved: false,
    })


  onSave = () => {
    const { text } = this.state;
    this.props.db.createNote(text)
      .then(() => this.setState({text: ""}))
      .catch(error => this.setState({error}))
  }

  dismissError = () => this.setState({error: null})


  render() {
    return (
      <div class="container">
        <div class="field">
          <textarea class={cls("textarea", {"is-success": this.state.draftSaved})}
            ref={ref => this.textarea = ref}
            onInput={this.onText}
            value={this.state.text}/>
        </div>
        <div class="buttons">
          <button class="button is-primary"
            disabled={!this.state.text}
            onClick={this.onSave}>
            Save
          </button>
        </div>
      {this.state.error &&
        <div class="notification is-warning" onClick={this.dismissError}>
          <button class="delete" onClick={this.dismissError}></button>
          Sorry!<br/>
          We could not save your note to the local storage: {this.state.error}.
        </div>
      }
      </div>
    );
  }
}
