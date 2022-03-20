import { shallowMount } from '@vue/test-utils';
import LoadingAlert from './LoadingAlert';

describe('LoadingAlert.vue', () => {
  it('renders', () => {
    const w = shallowMount(LoadingAlert, {
      props: {
        message: 'Test',
      }
    });
    expect(w.find('#component-loadingalert-message').text()).toBe('Test');
  });
});
