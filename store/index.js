export const state = () => ({
  pageNo: 0,
});

export const mutations = {
  page(state, pageNo) {
    state.pageNo = pageNo;
  },
};
