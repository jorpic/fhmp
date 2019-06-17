// Navbar allows to switch current app section.

import cls from "classnames";
import {h, Component} from "preact";
import {Link} from "preact-router";


export default class Navbar extends Component {
  constructor(props) {
    super(props);
    this.state = {
      isMenuActive: false,
    };
  }


  toggleBurger = () => this.setState({
    isMenuActive: !this.state.isMenuActive
  })


  render() {
    const isActiveCls = cls({"is-active": this.state.isMenuActive});
    return (
      <nav class="navbar has-shadow is-fixed-top"
        role="navigation" aria-label="main navigation"
      >
        <div class="navbar-brand">
          <div class="navbar-item">FHMP</div>
          <a class={cls("navbar-burger burger", isActiveCls)}
            role="button" aria-label="menu" aria-expanded="false"
            onClick={this.toggleBurger}
          >
            <span aria-hidden="true" />
            <span aria-hidden="true" />
            <span aria-hidden="true" />
          </a>
        </div>

        <div class={cls("navbar-menu", isActiveCls)}>
          <div class="navbar-start">
            <Item url="/new" icon="fas fa-seedling" text="Add Note" />
            <Item url="/list" icon="fas fa-list" text="List" />
            <Item url="/review" icon="fas fa-bong" text="Review" />
            <Item url="/config" icon="fas fa-cog" text="Config" />
            <Item url="/sync" icon="fas fa-sync" text="Sync" />
          </div>
        </div>
      </nav>
    );
  }
}


const Item = props => {
  const isActive = window.location.pathname.startsWith(props.url);
  return (
    <Link
      class={cls("navbar-item", {"is-active": isActive})}
      href={props.url}
    >
      <span class="icon"><i class={props.icon} /></span>
      <span>{props.text}</span>
    </Link>);
};
