import { h, Component } from "preact";

export default class Create extends Component {
  constructor(props) {
    super(props);
    this.state = {
      error: null,
      text: ""
    };
  }

  // TODO: load draft from IDB when mounted
  // TODO: save draft to IDB each 10 seconds

  focus() {
    this.textarea.focus();
  }

  onText = ev => this.setState({text: ev.target.value})

  onSave = () => {
    const { text } = this.state;
    this.props.onSave(text)
      .then(() => this.setState({text: ""}))
      .catch(error => this.setState({error}))
  }

  dismissError = () => this.setState({error: null})


  render() {
    return (
      <div class="container">
        <div class="field">
          <textarea class="textarea"
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
