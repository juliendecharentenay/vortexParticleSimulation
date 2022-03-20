import { shallowMount } from '@vue/test-utils';
import ErrorAlert from './ErrorAlert';

describe('ErrorAlert.vue', () => {
  it('renders', () => {
    const w = shallowMount(ErrorAlert, {
      props: {
        message: 'Test',
        error: {a: 'a', b: 'b'}
      }
    });
    expect(w.find('#component-erroralert-message').text()).toBe('Test');
  });

  it('emits on_close', async () => {
    const w = shallowMount(ErrorAlert, {
      props: {
        message: 'Test',
        error: {a: 'a', b: 'b'}
      }
    });
    await w.find('#component-erroralert-dismissbtn').trigger('click');
    expect(w.emitted().on_close.length).toBe(1);
  });
});
