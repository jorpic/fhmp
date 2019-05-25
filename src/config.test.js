import {nextReviewInterval} from "./config";

const hour = 60 * 60 * 1000;
const day = 24 * hour;

function t(expected, actual, result, next) {
  const res = nextReviewInterval(expected, actual, result) / day;
  expect(res).toBe(next);
}


test("first review was easy", () => t(0, 1 * hour, "easy", 1));
test("first review was hard", () => t(0, 1 * hour, "hard", 1));

test("first review after a long time was easy", () => t(0, 300 * day, "easy", 144)); // NB
test("first review after a long time was hard", () => t(0, 300 * day, "hard", 1));

test("increase interval", () => t(3 * day, 3 * day + 12 * hour, "easy", 5));
test("decrease interval", () => t(3 * day, 3 * day + 12 * hour, "hard", 2));

test("upper saturation", () => t(144 * day, 144 * day + 12 * hour, "easy", 144));
test("lower saturation", () => t(1 * day, 1 * day + 12 * hour, "hard", 1));

// NB
test("premature review easy", () => t(5 * day, 1 * day, "easy", 5));
test("premature review hard 1", () => t(5 * day, 1 * day, "hard", 1));
test("premature review hard 2", () => t(5 * day, 2 * day + 2 * hour, "hard", 2));
