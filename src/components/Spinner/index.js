import { h, Component } from 'preact';
import style from './style';

// This is adopted from the code by Fran Pérez.
// Original version is here https://codepen.io/mrrocks/pen/EiplA
// Seems that it is MIT licensed (see https://blog.codepen.io/legal/licensing).

export default class Spinner extends Component {
  render() {
    return (
      <div class={style.spinnerWrapper}>
        <svg
            class={style.spinner}
            width="65px"
            height="65px"
            viewBox="0 0 66 66"
            xmlns="http://www.w3.org/2000/svg">
          <circle
              class={style.path}
              fill="none"
              stroke-width="6"
              stroke-linecap="round"
              cx="33" cy="33"
              r="30">
          </circle>
        </svg>
      </div>
    );
  }
}
