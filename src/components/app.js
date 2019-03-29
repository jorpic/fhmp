import "../style";
import "bulma/css/bulma.css";
import { h, Component } from "preact";

import {Tab, Tabs} from "./Tabs";

export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      text: "hello"
    };
  }

  componentDidMount() {
    this.textarea && this.textarea.focus();
  }

  onText = ev => this.setState({text: ev.target.value})

  render() {
    return (
      <div class="section">
        <div class="container">
          <Tabs>
            <Tab name="New">
              <textarea class="textarea"
                ref={ref => this.textarea = ref}
                onChange={this.onText}
                value={this.state.text}/>
            </Tab>
            <Tab name="List">
              List
            </Tab>
          </Tabs>
        </div>
      </div>
    );
  }
}
