import { shallowMount } from '@vue/test-utils';
import NumericsProperties from './NumericsProperties.vue';

describe("NumericsProperties.vue", () => {
  it("renders", async () => {
    const wrapper = shallowMount(NumericsProperties, {
      props: {
        modelValue: {
          n_vortons: 100
        }
      }
    });

    await wrapper.vm.$nextTick();
    expect(wrapper.find('#n-vortons').element.value).toEqual("100");

    await wrapper.find('#n-vortons').setValue("10");
    const update = wrapper.emitted('update:modelValue');
    expect(update).toHaveLength(1);
    expect(update[0][0].n_vortons).toEqual(10);

  });
});

