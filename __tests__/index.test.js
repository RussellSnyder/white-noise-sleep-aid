const fs = require('fs');
const path = require('path');
const html = fs.readFileSync(path.resolve(__dirname, '../index.html'), 'utf8');

jest
  .dontMock('fs');

const base_id = "fade-in";
const seconds_id = `${base_id}-seconds`;
const minutes_id = `${base_id}-minutes`;

describe('Fade In', function () {
  beforeEach(() => {
    document.documentElement.innerHTML = html.toString();
  });

  afterEach(() => {
    jest.resetModules();
  });

  describe("initial state", () => {
    it('fade in second elements exists', function () {
      expect(getSecondsInput()).toBeTruthy();
      expect(getSecondsUp()).toBeTruthy();
      expect(getSecondsDown()).toBeTruthy();
    });

    it('fade in minute elements exists', function () {
      expect(getMinutesInput()).toBeTruthy();
      expect(getMinutesUp()).toBeTruthy();
      expect(getMinutesDown()).toBeTruthy();
    });
  })

  describe("Interactions", () => {
    describe("Seconds", () => {
      it('changes the value when text changes', () => {
        getSecondsInput().value = 5;
        expect(parseInt(getSecondsInput().value)).toBe(5);
      })
 
      it('should increment 1 when + is clicked', () => {
        getSecondsInput().value = 7;
        getSecondsUp().click();
        expect(parseInt(getSecondsInput().value)).toBe(8);
      }) 
    })
  })

  function getSecondsInput() {
    return document.getElementById(seconds_id);
  } 
  function getSecondsUp() {
    return document.getElementById(seconds_id + "+");
  } 
  function getSecondsDown() {
    return document.getElementById(seconds_id + "-");
  } 
  function getMinutesInput() {
    return document.getElementById(minutes_id);
  } 
  function getMinutesUp() {
    return document.getElementById(minutes_id + "+");
  } 
  function getMinutesDown() {
    return document.getElementById(minutes_id + "-");
  }
});