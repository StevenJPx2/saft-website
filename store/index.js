export const state = () => ({
  pageNo: 0,
});

export const mutations = {
  page(pageNo) {
    state.pageNo = pageNo;
  },
};
