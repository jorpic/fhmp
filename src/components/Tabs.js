import cls from "classnames";
import { h, Component } from "preact";

export class Tabs extends Component {
  constructor(props) {
    super(props);
    this.state = {
      active: 0
    };
  }

  setActive = (tab, i) => () => {
    tab.attributes.onActive && tab.attributes.onActive();
    this.setState({active: i});
  }

  render() {
    const mkTab = (tab, i) => (
      <li onClick={this.setActive(tab, i)}
          class={cls({"is-active": this.state.active == i})}>
        <a>
          {tab.attributes.icon
            ? <span class="icon"><i class={tab.attributes.icon}/></span>
            : ""}
          {tab.attributes.name}
        </a>
      </li>);
    const { children } = this.props;

    return (
      <div class="tabs-wrapper">
        <div class="tabs is-centered">
          <ul>
            {children.map(mkTab)}
          </ul>
        </div>
        {children[this.state.active]}
      </div>);
  }
}


export const Tab = ({children}) =>
    children.length == 1
      ? children[0]
      : (<div>{children}</div>);
