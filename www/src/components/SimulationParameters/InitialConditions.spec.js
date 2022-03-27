import { shallowMount } from '@vue/test-utils';
import InitialConditions from './InitialConditions.vue';

describe("InitialConditions.vue", () => {
  it("renders", async () => {
    const wrapper = shallowMount(InitialConditions, {
      props: {
        modelValue: {
          InitialConditionVortexRing: {
            center: {
              x: 0,
              y: 1,
              z: 2,
            },
            direction: {
              x: 3,
              y: 4,
              z: 5,
            },
            intensity: 6,
            radius: 7,
            thickness: 8,
          }
        }
      }
    });

    let update;

    await wrapper.vm.$nextTick();
    expect(wrapper.find('#center-x').element.value).toEqual("0");
    await wrapper.find('#center-x').setValue("10");
    update = wrapper.emitted('update:modelValue').slice(-1)[0];
    expect(update[0].InitialConditionVortexRing.center.x).toEqual(10);

    expect(wrapper.find('#center-y').element.value).toEqual("1");
    await wrapper.find('#center-y').setValue("11");
    update = wrapper.emitted('update:modelValue').slice(-1)[0];
    expect(update[0].InitialConditionVortexRing.center.y).toEqual(11);

    expect(wrapper.find('#center-z').element.value).toEqual("2");
    await wrapper.find('#center-z').setValue("12");
    update = wrapper.emitted('update:modelValue').slice(-1)[0];
    expect(update[0].InitialConditionVortexRing.center.z).toEqual(12);

    expect(wrapper.find('#direction-x').element.value).toEqual("3");
    await wrapper.find('#direction-x').setValue("13");
    update = wrapper.emitted('update:modelValue').slice(-1)[0];
    expect(update[0].InitialConditionVortexRing.direction.x).toEqual(13);

    expect(wrapper.find('#direction-y').element.value).toEqual("4");
    await wrapper.find('#direction-y').setValue("14");
    update = wrapper.emitted('update:modelValue').slice(-1)[0];
    expect(update[0].InitialConditionVortexRing.direction.y).toEqual(14);

    expect(wrapper.find('#direction-z').element.value).toEqual("5");
    await wrapper.find('#direction-z').setValue("15");
    update = wrapper.emitted('update:modelValue').slice(-1)[0];
    expect(update[0].InitialConditionVortexRing.direction.z).toEqual(15);

    expect(wrapper.find('#intensity').element.value).toEqual("6");
    await wrapper.find('#intensity').setValue("16");
    update = wrapper.emitted('update:modelValue').slice(-1)[0];
    expect(update[0].InitialConditionVortexRing.intensity).toEqual(16);

    expect(wrapper.find('#radius').element.value).toEqual("7");
    await wrapper.find('#radius').setValue("17");
    update = wrapper.emitted('update:modelValue').slice(-1)[0];
    expect(update[0].InitialConditionVortexRing.radius).toEqual(17);

    expect(wrapper.find('#thickness').element.value).toEqual("8");
    await wrapper.find('#thickness').setValue("18");
    update = wrapper.emitted('update:modelValue').slice(-1)[0];
    expect(update[0].InitialConditionVortexRing.thickness).toEqual(18);

  });
});

