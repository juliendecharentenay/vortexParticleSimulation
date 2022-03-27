import { shallowMount } from '@vue/test-utils';
import PhysicalProperties from './PhysicalProperties.vue';

describe("PhysicalProperties.vue", () => {
  it("renders", async () => {
    const wrapper = shallowMount(PhysicalProperties, {
      props: {
        modelValue: {
          viscosity: 0.1
        }
      }
    });

    await wrapper.vm.$nextTick();
    expect(wrapper.find('#viscosity').element.value).toEqual("0.1");

    await wrapper.find('#viscosity').setValue("0.5");
    const update = wrapper.emitted('update:modelValue').slice(-1)[0];
    expect(update[0].viscosity).toEqual(0.5);

  });
});

